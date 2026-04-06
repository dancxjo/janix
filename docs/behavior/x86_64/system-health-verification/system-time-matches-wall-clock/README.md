# ❌ Scenario: System Time matches Wall Clock

> Last run: 2026-04-05 17:45:26

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is running | ✅ | 5468ms | - - - |
| 2 | When I wait for the system to reach ready state | ✅ | 0ms | - - - |
| 3 | Then I should see a clock window displaying a ticking clock | ❌ | 31091ms | - [📜](./03/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[12426720669] [INFO] [kernel] [CPU0] BOOTFB: width=1920 height=1080 pitch=7680 bpp=32 format=Bgrx8888
[12432038652] [CONTRACT] [kernel] [CPU0] thing-os kernel starting...
[12435563547] [INFO] [bran::requests] [CPU0] Limine: Found 39 boot modules
[12437514276] [INFO] [bran::requests] [CPU0]   [0] /boot/sprout (cmdline='init') size=111120
[12438686502] [INFO] [bran::requests] [CPU0]   [1] /boot/bristle (cmdline='init') size=41456
[12439308552] [INFO] [bran::requests] [CPU0]   [2] /boot/rtc_cmos (cmdline='init') size=29248
[12440006040] [INFO] [bran::requests] [CPU0]   [3] /boot/clock (cmdline='') size=61968
[12440590437] [INFO] [bran::requests] [CPU0]   [4] /boot/taskman (cmdline='') size=70160
[12441166584] [INFO] [bran::requests] [CPU0]   [5] /boot/ps2_kbd (cmdline='init') size=25072
[12441765732] [INFO] [bran::requests] [CPU0]   [6] /boot/echo (cmdline='init') size=25072
[12442373460] [INFO] [bran::requests] [CPU0]   [7] /boot/bloom (cmdline='init') size=965128
[12442969011] [INFO] [bran::requests] [CPU0]   [8] /boot/ps2_mouse (cmdline='init') size=29168
[12443657523] [INFO] [bran::requests] [CPU0]   [9] /boot/display_bootfb (cmdline='init') size=29248
[12444308217] [INFO] [bran::requests] [CPU0]   [10] /boot/display_virtio_gpu (cmdline='init') size=53864
[12444984321] [INFO] [bran::requests] [CPU0]   [11] /boot/fontd (cmdline='init') size=188944
[12445602807] [INFO] [bran::requests] [CPU0]   [12] /boot/blossom (cmdline='init') size=123408
[12446214924] [INFO] [bran::requests] [CPU0]   [13] /boot/flytrap (cmdline='') size=291352
[12446803578] [INFO] [bran::requests] [CPU0]   [14] /boot/virtio_netd (cmdline='') size=49680
[12447412263] [INFO] [bran::requests] [CPU0]   [15] /boot/rtl8168d (cmdline='') size=45584
[12448000851] [INFO] [bran::requests] [CPU0]   [16] /boot/netd (cmdline='') size=147984
[12448586535] [INFO] [bran::requests] [CPU0]   [17] /boot/fetchd (cmdline='') size=41488
[12449203503] [INFO] [bran::requests] [CPU0]   [18] /boot/anther (cmdline='') size=586272
[12449806809] [INFO] [bran::requests] [CPU0]   [19] /boot/fortune (cmdline='') size=61968
[12450392724] [INFO] [bran::requests] [CPU0]   [20] /boot/photosynthesis (cmdline='') size=12648
[12451022430] [INFO] [bran::requests] [CPU0]   [21] /boot/ahci_disk (cmdline='') size=45664
[12451619499] [INFO] [bran::requests] [CPU0]   [22] /boot/iso9660d (cmdline='') size=53856
[12452235741] [INFO] [bran::requests] [CPU0]   [23] /boot/virtio_sound (cmdline='init') size=49648
[12452888217] [INFO] [bran::requests] [CPU0]   [24] /boot/hdaudio (cmdline='') size=37360
[12453471954] [INFO] [bran::requests] [CPU0]   [25] /boot/pci_stubd (cmdline='') size=29168
[12454070046] [INFO] [bran::requests] [CPU0]   [26] /boot/beeper (cmdline='init') size=33296
[12454669722] [INFO] [bran::requests] [CPU0]   [27] /boot/nectar (cmdline='init') size=176656
[12455287746] [INFO] [bran::requests] [CPU0]   [28] /boot/hello_std (cmdline='') size=16880
[12455901744] [INFO] [bran::requests] [CPU0]   [29] /boot/stdio_demo (cmdline='') size=16880
[12456503763] [INFO] [bran::requests] [CPU0]   [30] /boot/telnetd (cmdline='') size=57872
[12457083837] [INFO] [bran::requests] [CPU0]   [31] /assets/wallpapers/clouds.bmp (cmdline='') size=786486
[12457767267] [INFO] [bran::requests] [CPU0]   [32] /assets/wallpapers/flower.bmp (cmdline='') size=8386614
[12458474193] [INFO] [bran::requests] [CPU0]   [33] /assets/wallpapers/leather.bmp (cmdline='') size=1179702
[12459169305] [INFO] [bran::requests] [CPU0]   [34] /assets/wallpapers/linen.bmp (cmdline='') size=4718646
[12459854319] [INFO] [bran::requests] [CPU0]   [35] /assets/fonts/NotoSans-Regular.ttf (cmdline='') size=569208
[12460569495] [INFO] [bran::requests] [CPU0]   [36] /assets/themes/genie_circles.wasm (cmdline='') size=2824
[12461267412] [INFO] [bran::requests] [CPU0]   [37] /assets/cursors/future/default.svg (cmdline='') size=3051
[12461984832] [INFO] [bran::requests] [CPU0]   [38] /boot/locale.conf (cmdline='') size=85
[12463263417] [INFO] [kernel::memory] [CPU0] Memory map has 64 entries
[12464651694] [INFO] [kernel::memory] [CPU0]   [0] 0x0 - 0x87000 (Usable)
[12465431682] [INFO] [kernel::memory] [CPU0]   [1] 0x87000 - 0x88000 (Reserved)
[12465969582] [INFO] [kernel::memory] [CPU0]   [2] 0x88000 - 0xa0000 (Usable)
[12466476561] [INFO] [kernel::memory] [CPU0]   [3] 0x100000 - 0x800000 (Usable)
[12466993605] [INFO] [kernel::memory] [CPU0]   [4] 0x800000 - 0x808000 (Other)
[12467618658] [INFO] [kernel::memory] [CPU0]   [5] 0x808000 - 0x80b000 (Usable)
[12468135999] [INFO] [kernel::memory] [CPU0]   [6] 0x80b000 - 0x80c000 (Other)
[12468658158] [INFO] [kernel::memory] [CPU0]   [7] 0x80c000 - 0x811000 (Usable)
[12469173123] [INFO] [kernel::memory] [CPU0]   [8] 0x811000 - 0x900000 (Other)
[12469680333] [INFO] [kernel::memory] [CPU0]   [9] 0x900000 - 0x1780000 (Reserved)
[12470213514] [INFO] [kernel::memory] [CPU0]   [10] 0x1780000 - 0x7797d000 (Usable)
[12470755011] [INFO] [kernel::memory] [CPU0]   [11] 0x7797d000 - 0x779e1000 (Reserved)
[12471314724] [INFO] [kernel::memory] [CPU0]   [12] 0x779e1000 - 0x779e2000 (Other)
[12471872424] [INFO] [kernel::memory] [CPU0]   [13] 0x779e2000 - 0x779e3000 (Reserved)
[12472431708] [INFO] [kernel::memory] [CPU0]   [14] 0x779e3000 - 0x779e4000 (Other)
[12472972941] [INFO] [kernel::memory] [CPU0]   [15] 0x779e4000 - 0x779e5000 (Reserved)
[12473530740] [INFO] [kernel::memory] [CPU0]   [16] 0x779e5000 - 0x779e6000 (Other)
[12474070587] [INFO] [kernel::memory] [CPU0]   [17] 0x779e6000 - 0x779e7000 (Reserved)
[12474625284] [INFO] [kernel::memory] [CPU0]   [18] 0x779e7000 - 0x77a72000 (Other)
[12475174074] [INFO] [kernel::memory] [CPU0]   [19] 0x77a72000 - 0x77a73000 (Reserved)
[12475729398] [INFO] [kernel::memory] [CPU0]   [20] 0x77a73000 - 0x77ef4000 (Other)
[12476266902] [INFO] [kernel::memory] [CPU0]   [21] 0x77ef4000 - 0x77ef5000 (Reserved)
[12476822622] [INFO] [kernel::memory] [CPU0]   [22] 0x77ef5000 - 0x78016000 (Other)
[12477359697] [INFO] [kernel::memory] [CPU0]   [23] 0x78016000 - 0x78017000 (Reserved)
[12477915615] [INFO] [kernel::memory] [CPU0]   [24] 0x78017000 - 0x78817000 (Other)
[12478468002] [INFO] [kernel::memory] [CPU0]   [25] 0x78817000 - 0x78818000 (Reserved)
[12479021841] [INFO] [kernel::memory] [CPU0]   [26] 0x78818000 - 0x788d9000 (Other)
[12479559048] [INFO] [kernel::memory] [CPU0]   [27] 0x788d9000 - 0x788da000 (Reserved)
[12480113250] [INFO] [kernel::memory] [CPU0]   [28] 0x788da000 - 0x788e9000 (Other)
[12480648411] [INFO] [kernel::memory] [CPU0]   [29] 0x788e9000 - 0x788ea000 (Reserved)
[12481203273] [INFO] [kernel::memory] [CPU0]   [30] 0x788ea000 - 0x788ef000 (Other)
[12481753845] [INFO] [kernel::memory] [CPU0]   [31] 0x788ef000 - 0x788f0000 (Reserved)
[12482311710] [INFO] [kernel::memory] [CPU0]   [32] 0x788f0000 - 0x788f5000 (Other)
[12482850237] [INFO] [kernel::memory] [CPU0]   [33] 0x788f5000 - 0x788f6000 (Reserved)
[12483581319] [INFO] [kernel::memory] [CPU0]   [34] 0x788f6000 - 0x78922000 (Other)
[12484129020] [INFO] [kernel::memory] [CPU0]   [35] 0x78922000 - 0x78924000 (Reserved)
[12484690383] [INFO] [kernel::memory] [CPU0]   [36] 0x78924000 - 0x7892d000 (Other)
[12485255970] [INFO] [kernel::memory] [CPU0]   [37] 0x7892d000 - 0x7892f000 (Reserved)
[12485817333] [INFO] [kernel::memory] [CPU0]   [38] 0x7892f000 - 0x78937000 (Other)
[12486356058] [INFO] [kernel::memory] [CPU0]   [39] 0x78937000 - 0x78938000 (Reserved)
[12486910887] [INFO] [kernel::memory] [CPU0]   [40] 0x78938000 - 0x78942000 (Other)
[12487449150] [INFO] [kernel::memory] [CPU0]   [41] 0x78942000 - 0x78943000 (Reserved)
[12488003649] [INFO] [kernel::memory] [CPU0]   [42] 0x78943000 - 0x78950000 (Other)
[12488555343] [INFO] [kernel::memory] [CPU0]   [43] 0x78950000 - 0x78952000 (Reserved)
[12489112185] [INFO] [kernel::memory] [CPU0]   [44] 0x78952000 - 0x78960000 (Other)
[12489648831] [INFO] [kernel::memory] [CPU0]   [45] 0x78960000 - 0x78961000 (Reserved)
[12490204122] [INFO] [kernel::memory] [CPU0]   [46] 0x78961000 - 0x7896d000 (Other)
[12490743309] [INFO] [kernel::memory] [CPU0]   [47] 0x7896d000 - 0x7896e000 (Reserved)
[12491298567] [INFO] [kernel::memory] [CPU0]   [48] 0x7896e000 - 0x78972000 (Other)
[12491852142] [INFO] [kernel::memory] [CPU0]   [49] 0x78972000 - 0x78973000 (Reserved)
[12492405651] [INFO] [kernel::memory] [CPU0]   [50] 0x78973000 - 0x78983000 (Other)
[12492958434] [INFO] [kernel::memory] [CPU0]   [51] 0x78983000 - 0x78984000 (Reserved)
[12493523361] [INFO] [kernel::memory] [CPU0]   [52] 0x78984000 - 0x78a14000 (Other)
[12494063835] [INFO] [kernel::memory] [CPU0]   [53] 0x78a14000 - 0x78a15000 (Reserved)
[12494622228] [INFO] [kernel::memory] [CPU0]   [54] 0x78a15000 - 0x78a20000 (Other)
[12495239493] [INFO] [kernel::memory] [CPU0]   [55] 0x78a20000 - 0x78a21000 (Reserved)
[12495799008] [INFO] [kernel::memory] [CPU0]   [56] 0x78a21000 - 0x78a46000 (Other)
[12496338657] [INFO] [kernel::memory] [CPU0]   [57] 0x78a46000 - 0x78a47000 (Reserved)
[12496896753] [INFO] [kernel::memory] [CPU0]   [58] 0x78a47000 - 0x78a53000 (Other)
[12497439834] [INFO] [kernel::memory] [CPU0]   [59] 0x78a53000 - 0x78a54000 (Reserved)
[12498017136] [INFO] [kernel::memory] [CPU0]   [60] 0x78a54000 - 0x78a61000 (Other)
[12498556620] [INFO] [kernel::memory] [CPU0]   [61] 0x78a61000 - 0x78a62000 (Reserved)
[12499112802] [INFO] [kernel::memory] [CPU0]   [62] 0x78a62000 - 0x78aaa000 (Other)
[12499652055] [INFO] [kernel::memory] [CPU0]   [63] 0x78aaa000 - 0x78aab000 (Reserved)
[12500423925] [INFO] [kernel::memory] [CPU0] HHDM Offset: 0xffff800000000000
[12732658521] [CONTRACT] [kernel::memory] [CPU0] Frame allocator initialized with 485781 free frames
[12743308776] [INFO] [bran::arch] [CPU0] IOAPIC: hhdm=0xffff800000000000
[12748057377] [INFO] [bran::arch] [CPU0] IOAPIC: Disabling legacy PIC...
[12749302071] [INFO] [bran::arch] [CPU0] IOAPIC: PIC disabled OK
[12750142977] [INFO] [bran::arch] [CPU0] IOAPIC: RSDP virt=0x7f77e014
[12754195377] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: total length 144
[12756136404] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77802c
[12757205340] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778034
[12757878441] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77803c
[12758558373] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778044
[12759215667] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 1, len 12 at 0xffff80007f77804c
[12760163889] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778058
[12761107491] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778062
[12761801052] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f77806c
[12762467553] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778076
[12763129302] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778080
[12763829595] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 4, len 6 at 0xffff80007f77808a
[12765115737] [INFO] [bran::arch] [CPU0] IOAPIC: MADT parsed OK
[12766150386] [INFO] [bran::arch] [CPU0] SMP: Found 4 CPUs (CPU_COUNT now = 4)
[12766852329] [INFO] [bran::arch] [CPU0] IOAPIC: Found at phys 0xfec00000, GSI base 0
[12768423921] [INFO] [bran::arch] [CPU0] IOAPIC: Registers initialized
[12769428474] [INFO] [bran::arch] [CPU0] IOAPIC: Local APIC enabled (SVR=0x1FF, TPR=0)
[12770435007] [INFO] [bran::arch] [CPU0] IOAPIC: version 0x20, 24 redir entries
[12772121835] [INFO] [bran::arch] [CPU0] IOAPIC: All pins masked
[12773647854] [INFO] [bran::arch] [CPU0] IOAPIC: IRQ1 -> GSI 1 -> 0x21
[12774418140] [INFO] [bran::arch] [CPU0] IOAPIC: IRQ12 -> GSI 12 -> 0x2C
[12774966105] [INFO] [bran::arch] [CPU0] IOAPIC: Init complete
[12775708572] [CONTRACT] [kernel] [CPU0] Initializing global allocator...
[13144012893] [INFO] [kernel::memory::global_alloc] [CPU0] Global allocator initialized (LinkedHeap, 32MB)
[13145022231] [CONTRACT] [kernel] [CPU0] Seeding entropy pool...
[13148267286] [INFO] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (NOT seeded)
[13149131721] [INFO] [kernel::entropy] [CPU0] ENTROPY: marked seeded with weak entropy (timer-only fallback)
[13149891777] [CONTRACT] [kernel] [CPU0] Initializing SIMD...
[13151420370] [CONTRACT] [kernel] [CPU0] Initializing tasking...
[13163911497] [INFO] [kernel::sched] [CPU0]   Acquiring scheduler lock...
[13165761741] [INFO] [kernel::sched] [CPU0]   Lock acquired, checking if initialized...
[13166937069] [INFO] [kernel::sched] [CPU0]   Allocating scheduler...
[13169063325] [INFO] [kernel::sched] [CPU0]   Leaking scheduler...
[13169965050] [INFO] [kernel::sched] [CPU0]   Initializing boot task...
[13172861724] [INFO] [kernel::sched] [CPU0]   Creating boot task...
[13181383809] [INFO] [kernel::sched] [CPU0]   Creating idle tasks...
[13183533099] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 1 assigned to CPU 0
[13197592023] [INFO] [kernel::sched] [CPU0]   Boot task initialized
[13198559550] [INFO] [kernel::sched] [CPU0]   Storing scheduler pointer...
[13215676089] [INFO] [kernel::sched] [CPU0]   Initialized 4 event ring(s)
[13216654341] [CONTRACT] [kernel::sched] [CPU0] Scheduler initialized
[13219175673] [CONTRACT] [alloc] [CPU0] MEDIUM ALLOC #1: 224 KB align=8 total=0MB
[13220856132] [INFO] [kernel::task] [CPU0]   Creating graph worker tasks...
[13222300179] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 2 assigned to CPU 0
[13224990834] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 3 assigned to CPU 0
[13226195532] [INFO] [kernel] [CPU0] System initialized. Setting up preemption timer (100Hz)...
[13261224504] [INFO] [bran::arch::x86_64] [CPU0] LAPIC: calibrated timer (62403500 ticks/sec), init_cnt=624035 for 100Hz
[13262899155] [INFO] [kernel] [CPU0] Kernel: Detected 4 CPUs. Starting 3 secondaries...
[13263734451] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting 3 secondary CPUs...
[13264912749] [INFO] [bran::arch::x86_64] [CPU0] SMP: Initializing trampoline at 0x8000
[13270753452] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 1 (APIC 1)
[13302434937] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 1 (APIC 1) is online
[13303536741] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 2 (APIC 2)
[13305404706] [INFO] [kernel] [CPU1] SMP: Entering kernel_secondary_entry for CPU 1
[13307635836] [INFO] [kernel::sched] [CPU1] SMP: CPU 1 online (triggered by scheduler spawn)
[13309712394] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 4 assigned to CPU 1
[13314530097] [INFO] [kernel::sched] [CPU1] SMP: Secondary CPU 1 online!
[13316877420] [INFO] [kernel::task] [CPU1] SMP: CPU 1 bootstrapped with idle task 4
[13333654389] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 2 (APIC 2) is online
[13334449755] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 3 (APIC 3)
[13335334287] [INFO] [kernel] [CPU2] SMP: Entering kernel_secondary_entry for CPU 2
[13336485096] [INFO] [kernel::sched] [CPU2] SMP: CPU 2 online (triggered by scheduler spawn)
[13337527005] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 5 assigned to CPU 2
[13338892809] [INFO] [kernel::sched] [CPU2] SMP: Secondary CPU 2 online!
[13339628082] [INFO] [kernel::task] [CPU2] SMP: CPU 2 bootstrapped with idle task 5
[13364655645] [INFO] [kernel] [CPU3] SMP: Entering kernel_secondary_entry for CPU 3
[13366074480] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 3 (APIC 3) is online
[13367600961] [INFO] [kernel::sched] [CPU3] SMP: CPU 3 online (triggered by scheduler spawn)
[13368921225] [INFO] [bran::arch::x86_64] [CPU0] SMP: Secondary CPU startup complete
[13370271618] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 6 assigned to CPU 3
[13371645012] [INFO] [kernel] [CPU0] Kernel: Secondary CPU bring-up complete.
[13372870599] [INFO] [kernel::sched] [CPU3] SMP: Secondary CPU 3 online!
[13373898714] [INFO] [kernel::task] [CPU3] SMP: CPU 3 bootstrapped with idle task 6
[13380841716] [INFO] [kernel::root] [CPU0] Spawning Root service...
[13382309226] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 7 assigned to CPU 1
[13384686018] [INFO] [kernel::root::boot_register] [CPU0] ROOT: boot registration begin (Census Phase 1 v0.2)
[13386011727] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 1. Host creation start
[13394652810] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
RM
[13397431938] [INFO] [kernel::root::service] [CPU1] ROOT: started once
[13398684651] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Initializing components...
[13400835030] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Graph initialized
[13402439028] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Journal initialized
[13403979171] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Interner initialized
[13424089767] [CONTRACT] [kernel::root::service] [CPU1] ROOT: LogSymbols initialized
[13427881203] [CONTRACT] [kernel::root::service] [CPU1] ROOT: BatchScratch initialized
[13429716432] [CONTRACT] [kernel::root::service] [CPU1] ROOT: QueryScratch initialized
[13431069201] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Entering main loop
[13457518437] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13476168420] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13478421264] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13481816370] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13483374696] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13485543159] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13487819631] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13489785870] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 1. Host registered: t1
[13490961957] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 2. Platform Bus start
[13492391517] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13499128071] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13500939144] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13503661479] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13505932209] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13508224092] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13510796607] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 2. Platform Bus done
[13511593656] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 3. Kernel start
[13523694393] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 3. Kernel done
[13524665847] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 4. Root Service start
[13531638153] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 4. Root Service done
[13533437214] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 5. CPUs start
[13560648948] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 5. CPUs done
[13561459824] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 6. Memory Ranges start
[13910082219] [CONTRACT] [alloc] [CPU1] MEDIUM ALLOC #2: 120 KB align=8 total=1MB
[14433384669] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 6. Memory Ranges done
[14464975899] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=1000 nodes=331 watches=0 history=498 journal=423 symbols=51 drops=0
[14502802842] [CONTRACT] [alloc] [CPU1] MEDIUM ALLOC #3: 240 KB align=8 total=1MB
[15852305667] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=2000 nodes=447 watches=0 history=961 journal=769 symbols=95 drops=0
[16561719867] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: filesystem bootstrap: /initrd/ with 39 files
[16653892068] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: Census Phase 2: PCI
[16654999152] [INFO] [kernel::root::pci] [CPU0] PCI: Starting enumeration...
[16755962520] [INFO] [kernel::root::pci] [CPU0] PCI: 00:00.0 8086:29c0 Intel Corporation 82G33/G31/P35/P31 Express DRAM Controller class=06:00 prog_if=00 rev=00
[16815564216] [INFO] [kernel::root::pci] [CPU0] PCI: 00:01.0 1af4:1050 Red Hat, Inc. Virtio 1.0 GPU class=03:00 prog_if=00 rev=01
[16847190426] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR0: phys=0x80000000 size=0x800000
[16848176763] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR2: phys=0xc000000000 size=0x4000
[16848815676] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR4: phys=0x80882000 size=0x1000
[16852685652] [INFO] [kernel::root::pci] [CPU0] PCI: Found virtio display controller at 00:01.0
[16869803841] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO notify_cfg BAR2 offset=0x3000 mult=4
[16886987568] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO common_cfg BAR2 offset=0x1000
[16889447850] [INFO] [kernel::root::pci] [CPU0] PCI: Registered virtio GPU (graph_id=1086, idx=1) BAR0=0x80000000
[16943250159] [INFO] [kernel::root::pci] [CPU0] PCI: 00:02.0 1af4:1000 Red Hat, Inc. Virtio network device class=02:00 prog_if=00 rev=00
[16966378539] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR1: phys=0x80881000 size=0x1000
[16967438829] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR4: phys=0xc000004000 size=0x4000
[16972445688] [INFO] [kernel::root::pci] [CPU0] PCI: Found virtio network controller at 00:02.0
[16997489784] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO notify_cfg BAR4 offset=0x3000 mult=4
[17015724858] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO common_cfg BAR4 offset=0x0
[17023310799] [INFO] [kernel::root::pci] [CPU0] PCI: Registered virtio network (graph_id=1120, idx=2) BAR4=0xc000004000
[17024346636] [INFO] [kernel::root::pci] [CPU0] VirtIO-Net: Device registered for userspace driver
[17088927702] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.0 8086:2918 Intel Corporation 82801IB (ICH9) LPC Interface Controller class=06:01 prog_if=00 rev=02
[17091115569] [INFO] [kernel::root::pci] [CPU0] PCI: Found LPC/ISA bridge at 00:1f.0
[17175838482] [INFO] [kernel::root::pci] [CPU0] LPC: Created Legacy IO bus with CMOS and PS/2 controller
[17237550660] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.2 8086:2922 Intel Corporation 82801IR/IO/IH (ICH9R/DO/DH) 6 port SATA Controller [AHCI mode] class=01:06 prog_if=01 rev=02
[17250067659] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR5: phys=0x80880000 size=0x1000
[17254300206] [INFO] [kernel::root::pci] [CPU0] PCI: Found AHCI SATA controller at 00:1f.2
[17256644592] [INFO] [kernel::root::pci] [CPU0] PCI: Registered AHCI controller (graph_id=1193, idx=5) BAR5=0x80880000
[17275467627] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=3000 nodes=584 watches=0 history=1024 journal=1024 symbols=181 drops=0
[17323718346] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.3 8086:2930 Intel Corporation 82801I (ICH9 Family) SMBus Controller class=0c:05 prog_if=00 rev=02
[17328860274] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: registered items. host=t1 kernel=tb
[17329829385] [CONTRACT] [kernel] [CPU0] KERNEL: root census complete: host=t1 kernel=tb root=te
[17330670192] [CONTRACT] [kernel] [CPU0] Kernel: Enumerating 39 boot modules...
[17331538752] [CONTRACT] [kernel] [CPU0]   [0] name='/boot/sprout' cmdline='init' size=111120
[17332199148] [CONTRACT] [kernel] [CPU0]   [1] name='/boot/bristle' cmdline='init' size=41456
[17332836312] [CONTRACT] [kernel] [CPU0]   [2] name='/boot/rtc_cmos' cmdline='init' size=29248
[17333491725] [CONTRACT] [kernel] [CPU0]   [3] name='/boot/clock' cmdline='' size=61968
[17334086880] [CONTRACT] [kernel] [CPU0]   [4] name='/boot/taskman' cmdline='' size=70160
[17334694806] [CONTRACT] [kernel] [CPU0]   [5] name='/boot/ps2_kbd' cmdline='init' size=25072
[17335333488] [CONTRACT] [kernel] [CPU0]   [6] name='/boot/echo' cmdline='init' size=25072
[17335972533] [CONTRACT] [kernel] [CPU0]   [7] name='/boot/bloom' cmdline='init' size=965128
[17336624283] [CONTRACT] [kernel] [CPU0]   [8] name='/boot/ps2_mouse' cmdline='init' size=29168
[17337272139] [CONTRACT] [kernel] [CPU0]   [9] name='/boot/display_bootfb' cmdline='init' size=29248
[17337951972] [CONTRACT] [kernel] [CPU0]   [10] name='/boot/display_virtio_gpu' cmdline='init' size=53864
[17338663089] [CONTRACT] [kernel] [CPU0]   [11] name='/boot/fontd' cmdline='init' size=188944
[17339419383] [CONTRACT] [kernel] [CPU0]   [12] name='/boot/blossom' cmdline='init' size=123408
[17340165381] [CONTRACT] [kernel] [CPU0]   [13] name='/boot/flytrap' cmdline='' size=291352
[17340788025] [CONTRACT] [kernel] [CPU0]   [14] name='/boot/virtio_netd' cmdline='' size=49680
[17341448817] [CONTRACT] [kernel] [CPU0]   [15] name='/boot/rtl8168d' cmdline='' size=45584
[17342070108] [CONTRACT] [kernel] [CPU0]   [16] name='/boot/netd' cmdline='' size=147984
[17342671467] [CONTRACT] [kernel] [CPU0]   [17] name='/boot/fetchd' cmdline='' size=41488
[17343304605] [CONTRACT] [kernel] [CPU0]   [18] name='/boot/anther' cmdline='' size=586272
[17343917250] [CONTRACT] [kernel] [CPU0]   [19] name='/boot/fortune' cmdline='' size=61968
[17344892829] [CONTRACT] [kernel] [CPU0]   [20] name='/boot/photosynthesis' cmdline='' size=12648
[17345558373] [CONTRACT] [kernel] [CPU0]   [21] name='/boot/ahci_disk' cmdline='' size=45664
[17346349185] [CONTRACT] [kernel] [CPU0]   [22] name='/boot/iso9660d' cmdline='' size=53856
[17347386012] [CONTRACT] [kernel] [CPU0]   [23] name='/boot/virtio_sound' cmdline='init' size=49648
[17348296284] [CONTRACT] [kernel] [CPU0]   [24] name='/boot/hdaudio' cmdline='' size=37360
[17348922426] [CONTRACT] [kernel] [CPU0]   [25] name='/boot/pci_stubd' cmdline='' size=29168
[17349547182] [CONTRACT] [kernel] [CPU0]   [26] name='/boot/beeper' cmdline='init' size=33296
[17350209063] [CONTRACT] [kernel] [CPU0]   [27] name='/boot/nectar' cmdline='init' size=176656
[17351086467] [CONTRACT] [kernel] [CPU0]   [28] name='/boot/hello_std' cmdline='' size=16880
[17351714853] [CONTRACT] [kernel] [CPU0]   [29] name='/boot/stdio_demo' cmdline='' size=16880
[17352347991] [CONTRACT] [kernel] [CPU0]   [30] name='/boot/telnetd' cmdline='' size=57872
[17352978060] [CONTRACT] [kernel] [CPU0]   [31] name='/assets/wallpapers/clouds.bmp' cmdline='' size=786486
[17353704258] [CONTRACT] [kernel] [CPU0]   [32] name='/assets/wallpapers/flower.bmp' cmdline='' size=8386614
[17354437386] [CONTRACT] [kernel] [CPU0]   [33] name='/assets/wallpapers/leather.bmp' cmdline='' size=1179702
[17355174441] [CONTRACT] [kernel] [CPU0]   [34] name='/assets/wallpapers/linen.bmp' cmdline='' size=4718646
[17355898461] [CONTRACT] [kernel] [CPU0]   [35] name='/assets/fonts/NotoSans-Regular.ttf' cmdline='' size=569208
[17356669539] [CONTRACT] [kernel] [CPU0]   [36] name='/assets/themes/genie_circles.wasm' cmdline='' size=2824
[17357406198] [CONTRACT] [kernel] [CPU0]   [37] name='/assets/cursors/future/default.svg' cmdline='' size=3051
[17358149787] [CONTRACT] [kernel] [CPU0]   [38] name='/boot/locale.conf' cmdline='' size=85
[17359603074] [INFO] [kernel] [CPU0] Found init module: /boot/sprout (cmdline: 'init'), loading...
[17361299076] [INFO] [kernel::task::loader] [CPU0] Loading module: /boot/sprout
[17362065765] [DEBUG] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17369992761] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[17384000931] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=213000 exec=false
[17388262320] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=21a000 exec=false
[17397656298] [INFO] [kernel] [CPU0] Spawning sprout with registry at 0x600000...
[17398359264] [CONTRACT] [kernel] [CPU0] Spawning init process...
[17400403977] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 8 (user task/process) assigned to CPU 2
[17402938146] [CONTRACT] [kernel] [CPU0] Entering scheduler loop.
[17403824328] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600000
ThingOS Petals
type 'help' for commands

petals> [17410675887] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=8 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369013360 RFLAGS_BEFORE=130 CR3_BEFORE=50393088 fs_base=0 gs_base=18446744071564586608
[17413992750] [DEBUG] [kernel::sched::sleep] [CPU0] DIAG ctx_switch: cpu=0 from=0 to=2
[17431715268] [INFO] [sprout] [CPU2] [sprout] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ff990 rip=0x20139f rflags=0x206
[17434107471] [INFO] [sprout] [CPU2] SPROUT: v0.4 starting (Supervisor Mode)...
[17435514426] [INFO] [sprout::devtree] [CPU2] SPROUT: devtree::init entry (v0.2)
[17436788094] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 1: Find Host
[17444658693] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 2: HHDM
[17450349114] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 3: Platform Bus
[17453422668] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 4: Firmware
[17454467085] [INFO] [sprout::devtree] [CPU2] SPROUT: Finding ACPI...
[17457868461] [INFO] [sprout::devtree] [CPU2] SPROUT: Found 1 ACPI nodes
[17460983826] [INFO] [sprout::devtree] [CPU2] SPROUT: ACPI RSDP = 0x7f77e014
[17462119620] [INFO] [sprout::devtree] [CPU2] SPROUT: Finding DTB...
[17465909340] [INFO] [sprout::devtree] [CPU2] SPROUT: Found 0 DTB nodes
[17466978672] [INFO] [sprout::devtree] [CPU2] SPROUT: Init OK, returning context
[17468022957] [INFO] [sprout::devtree] [CPU2] SPROUT: build() called
[17469120834] [INFO] [sprout::devtree::x86_64] [CPU2] SPROUT: x86_64 platform enrichment... (v0.2)
[17472408558] [INFO] [sprout::devtree::x86_64] [CPU2] SPROUT: x86_64 enumerate done
[17473572105] [INFO] [sprout] [CPU2] SPROUT: About to create Supervisor...
[17474786703] [INFO] [sprout] [CPU2] SPROUT: Supervisor created, calling run_forever...
[17476589559] [INFO] [sprout::supervisor] [CPU2] SPROUT: Supervisor starting (phased mode)...
[17478364464] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 1] Hardware Discovery and Core Drivers
[17479677402] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovering modules...
[17483634003] [INFO] [sprout::supervisor] [CPU2] SPROUT: Found 39 modules
[17525685870] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[0] = '/boot/sprout'
[17532161592] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[1] = '/boot/bristle'
[17536591314] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[2] = '/boot/rtc_cmos'
[17541015888] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[3] = '/boot/clock'
[17544895170] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/clock
[17548909290] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[4] = '/boot/taskman'
[17553805962] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[5] = '/boot/ps2_kbd'
[17558401476] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[6] = '/boot/echo'
[17562989103] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[7] = '/boot/bloom'
[17567627286] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[8] = '/boot/ps2_mouse'
[17572524717] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[9] = '/boot/display_bootfb'
[17577966978] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[10] = '/boot/display_virtio_gpu'
[17582904999] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[11] = '/boot/fontd'
[17588728641] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[12] = '/boot/blossom'
[17593530504] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[13] = '/boot/flytrap'
[17598611547] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[14] = '/boot/virtio_netd'
[17604052488] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[15] = '/boot/rtl8168d'
[17610101256] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[16] = '/boot/netd'
[17615772075] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[17] = '/boot/fetchd'
[17620623966] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[18] = '/boot/anther'
[17625027519] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[19] = '/boot/fortune'
[17631245346] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[20] = '/boot/photosynthesis'
[17637228015] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[21] = '/boot/ahci_disk'
[17642862765] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[22] = '/boot/iso9660d'
[17648454582] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[23] = '/boot/virtio_sound'
[17653016832] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[24] = '/boot/hdaudio'
[17657971881] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[25] = '/boot/pci_stubd'
[17662863867] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[26] = '/boot/beeper'
[17668776873] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[27] = '/boot/nectar'
[17675987769] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[28] = '/boot/hello_std'
[17679677994] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/hello_std
[17683570872] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[29] = '/boot/stdio_demo'
[17689147542] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[30] = '/boot/telnetd'
[17694620790] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[31] = '/assets/wallpapers/clouds.bmp'
[17700341472] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[32] = '/assets/wallpapers/flower.bmp'
[17705123601] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[33] = '/assets/wallpapers/leather.bmp'
[17710197318] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[34] = '/assets/wallpapers/linen.bmp'
[17715274896] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[35] = '/assets/fonts/NotoSans-Regular.ttf'
[17720500776] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[36] = '/assets/themes/genie_circles.wasm'
[17725641483] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[37] = '/assets/cursors/future/default.svg'
[17731290423] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[38] = '/boot/locale.conf'
[17734519440] [INFO] [sprout::registry] [CPU2] SPROUT: Scanning boot modules...
[17753857704] [INFO] [sprout::registry] [CPU2] SPROUT: Registering driver 'dev.rtc.Cmos' -> '/boot/rtc_cmos' (fallback)
[17882180778] [INFO] [sprout::registry] [CPU2] SPROUT: Registry scan complete. Found 1 drivers.
[17889239247] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/pci_stubd
[17890115529] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17892024447] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17896724406] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[17898050775] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[17906775612] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 9 (user task/process) assigned to CPU 3
[17913591729] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db400
[17914568397] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned pci_stubd (PID=9)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[17915924103] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=9 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369078896 RFLAGS_BEFORE=130 CR3_BEFORE=50860032 fs_base=0 gs_base=18446744071564586640
[17920518891] [INFO] [pci_stubd] [CPU3] pci_stubd: starting pci-id matcher
[17921438964] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/rtc_cmos
[17922219612] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[17924637522] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[17929060149] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=204000 exec=false
[17930957220] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[17938150791] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 10 (user task/process) assigned to CPU 1
[17942296449] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned rtc_cmos (PID=10)
[17943102573] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x49b
[17944627437] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up storage pipeline...
[17945450556] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=10 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369144432 RFLAGS_BEFORE=130 CR3_BEFORE=50974720 fs_base=0 gs_base=18446744071564586576
[17951475234] [INFO] [rtc_cmos] [CPU1] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ffeb0 rip=0x201027 rflags=0x202
[17953254066] [INFO] [rtc_cmos] [CPU1] Starting... arg=49b
[17955352635] [INFO] [rtc_cmos] [CPU1] Serving device ID: ThingId([155, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[17960078499] [INFO] [rtc_cmos] [CPU1] RTC: 2026-04-06 01:02:40 = 1775437360 unix_secs
[17962089255] [INFO] [kernel::time] [CPU1] System clock anchored: unix_secs=1775437360, mono_ns=8980758666, offset=1775437351019241334ns
[17963731995] [INFO] [rtc_cmos] [CPU1] System clock anchored
[17975957604] [INFO] [rtc_cmos] [CPU1] RTC: Set sys.TimeState = 1 (Anchored)
[18018100188] [INFO] [rtc_cmos] [CPU1] Publishing time. Entering maintenance loop.
[18025157304] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ahci_disk
[18026119650] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[18028058829] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18034310019] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[18037096407] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20a000 exec=false
[18045431679] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 11 (user task/process) assigned to CPU 2
[18049279446] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ahci_disk (PID=11)
[18052680393] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18054344616] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=11 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369210672 RFLAGS_BEFORE=134 CR3_BEFORE=51089408 fs_base=0 gs_base=18446744071564586608
[18059543403] [INFO] [ahci_disk] [CPU2] AHCI: Starting AHCI/SATA disk driver v1
[18064464792] [INFO] [sprout::pipelines] [CPU2] SPROUT: Failed to spawn ata_disk: ENOENT
[18066369750] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/iso9660d
[18067287315] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[18069274014] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18076881405] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[18079446330] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[18087063819] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 12 (user task/process) assigned to CPU 3
[18090757641] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned iso9660d (PID=12)
[18091815786] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18093799977] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=12 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369277264 RFLAGS_BEFORE=134 CR3_BEFORE=51228672 fs_base=0 gs_base=18446744071564586640
[18098464659] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up audio driver...
[18099714237] [INFO] [iso9660d] [CPU3] ISO9660D: Starting ISO9660 mount service...
[18105280578] [INFO] [ahci_disk] [CPU2] AHCI: Found 6 PCI functions
[18106448316] [INFO] [iso9660d] [CPU3] ISO9660D: Found 0 block devices
[18108370203] [INFO] [iso9660d] [CPU3] ISO9660D: No ISO9660 filesystems found
[18109869756] [INFO] [iso9660d] [CPU3] ISO9660D: Entering service loop with 0 mounts
[18115589976] [INFO] [sprout::pipelines] [CPU2] SPROUT: No Sound device found
[18117488037] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up display pipeline...
[18129735492] [INFO] [sprout::pipelines] [CPU2] SPROUT: Using VirtIO GPU at 1920x1080
[18132079680] [INFO] [sprout::pipelines] [CPU2] SPROUT: Display backend: VirtIO-GPU (1920x1080 stride=7680)
[18631005129] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/display_virtio_gpu
[18632060898] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[18633929028] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18641572686] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[18644105370] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[18651671181] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 13 (user task/process) assigned to CPU 1
[18654140571] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00fab68
[18656223762] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned display driver '/display_virtio_gpu' (PID=13)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x30002
[18657741993] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=13 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369352672 RFLAGS_BEFORE=130 CR3_BEFORE=59662336 fs_base=0 gs_base=18446744071564586576
[18664105944] [INFO] [ahci_disk] [CPU2] AHCI: Found AHCI controller at PCI func ThingId([169, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[18665312259] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: starting (drv_req_r=2, drv_resp_w=3)
[18674422008] [INFO] [kernel::syscall::handlers::device] [CPU1] DEVICE: task 13 claimed device 1108 (handle 0)
[18675803619] [INFO] [ahci_disk] [CPU2] AHCI: BAR5=0x80880000
[18677172789] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 11 claimed device 1193 (handle 1)
[18678899712] [INFO] [ahci_disk] [CPU2] AHCI: Claimed PCI device ThingId([169, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) handle=1
[18683516379] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR5 phys=0x80880000 size=0x1000 -> virt=0x10000000
[18685488525] [INFO] [ahci_disk] [CPU2] AHCI: Mapped ABAR at 0x10000000
[18687690087] [INFO] [ahci_disk] [CPU2] AHCI: Version 1.0, 6 ports, 32 slots, 64-bit: true
[18689304777] [INFO] [ahci_disk] [CPU2] AHCI: Ports implemented: 0x3f
[18690061368] [INFO] [virtio_gpu] [CPU1] virtio_gpu: caps from graph - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[18691481490] [INFO] [ahci_disk] [CPU2] AHCI: DMA virt=0x20b000
[18692340249] [INFO] [kernel::syscall::handlers::device] [CPU1] DEVICE: Mapped BAR2 phys=0xc000000000 size=0x4000 -> virt=0x10001000
[18695265798] [INFO] [ahci_disk] [CPU2] AHCI: DMA phys=0x30c8000
[18696368031] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 0...
[18697917513] [INFO] [ahci_disk] [CPU2] AHCI: Port 0 - no device
[18698736474] [INFO] [virtio_gpu] [CPU1] virtio_gpu: device features=0x30000002 virgl=false
[18699705882] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 1...
[18700862994] [INFO] [ahci_disk] [CPU2] AHCI: Port 1 - no device
[18701725515] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 2...
[18702717099] [INFO] [ahci_disk] [CPU2] AHCI: Port 2 - SATAPI drive (sig=0xeb140101)
[18795673215] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: GPU initialized successfully
[18796893753] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up input pipeline (keyboard + mouse)...
[18798209727] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: Virgl 3D not supported, using 2D only
[18800105577] [INFO] [sprout::pipelines] [CPU2] SPROUT: Created kbd_raw port (w=7, r=8)
[18804387492] [INFO] [sprout::pipelines] [CPU2] SPROUT: Created mouse_raw port (w=9, r=10)
[18806915457] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ps2_kbd
[18809324127] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18811456488] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18815607030] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=203000 exec=false
[18817705203] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[18823607154] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: creating frame pool 1x 1920x1080 stride=7680 format=2
[18825896001] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 14 (user task/process) assigned to CPU 2
[18829446405] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ps2_kbd (PID=14)
[18837935886] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ps2_mouse
[18839355546] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18842020296] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18847452690] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[18849484401] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[18857276823] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 15 (user task/process) assigned to CPU 3
[18860798649] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb0105370
[18862206858] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ps2_mouse (PID=15)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x9
[18864289686] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=15 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369500080 RFLAGS_BEFORE=130 CR3_BEFORE=68235264 fs_base=0 gs_base=18446744071564586640
[18869207313] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/bristle
[18870594831] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18873160086] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18874585587] [INFO] [ps2_mouse] [CPU3] ps2_mouse: online (handle=9)
[18877026300] [INFO] [ps2_mouse] [CPU3] ps2_mouse: enabling aux port
[18879775794] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[18882861261] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[18884289798] [INFO] [ps2_mouse] [CPU3] ps2_mouse: controller cfg already correct (0x47)
[18886784433] [INFO] [ps2_mouse] [CPU3] ps2_mouse: sending RESET (0xFF)
[18889151754] [INFO] [ps2_mouse] [CPU3] ps2_mouse: reset ACK received (0xfa)
[18892362390] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 16 (user task/process) assigned to CPU 1
[18893902269] [INFO] [ps2_mouse] [CPU3] ps2_mouse: BAT passed (0xaa), ID 0x00 confirmed
[18896796369] [INFO] [ps2_mouse] [CPU3] ps2_mouse: setting sample rate (100)
[18897881178] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned bristle (PID=16)
[18899256156] [INFO] [ps2_mouse] [CPU3] ps2_mouse: setting resolution (3)
[18900662847] [INFO] [sprout::pipelines] [CPU2] SPROUT: Input broker ready (keyboard + mouse)
[18902545629] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up network stack...
[18903779103] [INFO] [ps2_mouse] [CPU3] ps2_mouse: status result = Some(0) Some(3) Some(100)
[18906252915] [INFO] [ps2_mouse] [CPU3] ps2_mouse: sending enable command (0xF4)
[18907506618] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00fff90
[18909005115] [INFO] [ps2_mouse] [CPU3] ps2_mouse: enable ACK received (0xfa)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x7
[18910886610] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=14 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369434000 RFLAGS_BEFORE=134 CR3_BEFORE=59830272 fs_base=0 gs_base=18446744071564586608
[18916119849] [INFO] [ps2_kbd] [CPU2] ps2_kbd: online (handle=7)
[19295592888] [INFO] [ahci_disk] [CPU2] AHCI: Registered ATAPI block device 1237 port=2 model='                                        ' rpc_port=5
[19298393895] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb0105370
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x8000a000b000d
[19299951429] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=16 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369582864 RFLAGS_BEFORE=130 CR3_BEFORE=68349952 fs_base=0 gs_base=18446744071564586576
[19305791439] [INFO] [bristle] [CPU1] BRISTLE_MAIN_ENTERED_WITH_LOGS_YAY
[19307961189] [INFO] [bristle] [CPU1] bristle: online (kbd=8, mouse=10, evt=11, echo=13)
[19310670555] [INFO] [bristle] [CPU1] bristle: created broadcast topic 0
[19322038725] [INFO] [bristle] [CPU1] bristle: registered in graph as svc.Input (id=1241)
[19323042453] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 3...
[19324237218] [INFO] [ahci_disk] [CPU2] AHCI: Port 3 - no device
[19325048754] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 4...
[19325929986] [INFO] [ahci_disk] [CPU2] AHCI: Port 4 - no device
[19327014597] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 5...
[19327777194] [INFO] [ahci_disk] [CPU2] AHCI: Port 5 - no device
[19328643213] [INFO] [ahci_disk] [CPU2] AHCI: Found 1 SATA disk(s)
[19329789006] [INFO] [ahci_disk] [CPU2] AHCI: Entering RPC service loop
[19336409235] [INFO] [ps2_kbd] [CPU2] ps2_kbd: created driver node 1240
[19338872190] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task subscribed to vector 0x21
[19340756886] [INFO] [ps2_kbd] [CPU2] ps2_kbd: subscribed to IRQ1 (vector 0x21)
[19344274686] [INFO] [sprout::pipelines] [CPU2] SPROUT: Found NIC device ThingId([117, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[19346389491] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/virtio_netd
[19347149184] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19348762422] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19355483070] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=208000 exec=false
[19357819272] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20b000 exec=false
[19365298425] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 17 (user task/process) assigned to CPU 2
[19368606675] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned virtio_netd (PID=17)
[19370541135] [INFO] [sprout::pipelines] [CPU2] SPROUT: spawn_netd start
[19372164075] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/netd
[19372879944] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19374564330] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19394177220] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=21d000 exec=false
[19398544968] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=223000 exec=false
[19412282010] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: frame pool ready (1 buffer)
[19419319953] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 18 (user task/process) assigned to CPU 3
[19422033015] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010a1b8
[19423592628] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned netd (PID=18)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19425215667] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=18 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369716560 RFLAGS_BEFORE=134 CR3_BEFORE=68890624 fs_base=0 gs_base=18446744071564586640
[19432334130] [INFO] [netd] [CPU3] NETD: Starting network stack service...
[19433489823] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb0105370
[19434553842] [INFO] [netd] [CPU3] NETD: Looking for virtio_netd driver service...
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x475
[19436304063] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=17 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369650768 RFLAGS_BEFORE=134 CR3_BEFORE=68747264 fs_base=0 gs_base=18446744071564586608
[19441852155] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Starting VirtIO-NET driver service...
[19443187170] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Searching for NIC device...
[19446316197] [INFO] [ps2_kbd] [CPU2] ps2_kbd: entering cooperative polling loop
[19448293458] [INFO] [ps2_kbd] [CPU2] ps2_kbd: using polling mode (10ms interval)
[19452209304] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Found NIC device t475
[19453808220] [INFO] [virtio::device] [CPU2] VirtIO: device::new(0x475) - claiming...
[19454943816] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 17 claimed device 1141 (handle 2)
[19456523031] [INFO] [virtio::device] [CPU2] VirtIO: claimed, handle=2
[19459151052] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 2] Starting Network Apps and Services
[19461183192] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up network apps...
[19462887048] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/anther
[19463608923] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19465266744] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19522081359] [INFO] [ps2_mouse] [CPU3] ps2_mouse: init done
[19523422512] [INFO] [kernel::syscall::handlers::device] [CPU3] DEVICE: task subscribed to vector 0x2c
[19525553520] [INFO] [ps2_mouse] [CPU3] ps2_mouse: subscribed to IRQ12 (vector 0x2c)
[19527578763] [INFO] [ps2_mouse] [CPU3] ps2_mouse: entering interrupt-driven loop
[19531865925] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=268000 exec=false
[19555920351] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=28e000 exec=false
[19563815040] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 19 (user task/process) assigned to CPU 1
[19567086099] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb0105370
[19568527473] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned anther (PID=19)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19570055109] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=19 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369782784 RFLAGS_BEFORE=134 CR3_BEFORE=69554176 fs_base=0 gs_base=18446744071564586576
[19573053918] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/nectar
[19573781271] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19575441369] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19576341444] [INFO] [anther] [CPU1] anther: Starting HTTP server (ThingOS anther v0.1)
[19579074933] [INFO] [anther] [CPU1] anther: Starting server mode on port 80...
[19583633124] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[19585247913] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[19602009867] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=226000 exec=false
[19605390651] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=22a000 exec=false
[19612663422] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 20 (user task/process) assigned to CPU 2
[19616178252] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned nectar (PID=20)
[19618760733] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/fetchd
[19619704401] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19621345887] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19627262952] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[19629833520] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[19636697124] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 21 (user task/process) assigned to CPU 3
[19639240566] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned fetchd (PID=21)
[19640316135] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010b5a8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19642206078] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=21 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369915168 RFLAGS_BEFORE=130 CR3_BEFORE=70488064 fs_base=0 gs_base=18446744071564586640
[19645578447] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/clock
[19646468589] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19647900954] [INFO] [fetchd] [CPU3] FETCHD: Starting IP address display...
[19648746447] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19649624082] [INFO] [fetchd] [CPU3] FETCHD: Waiting for UI Root (Compositor)...
[19657229163] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[19660847646] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20e000 exec=false
[19667949444] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 22 (user task/process) assigned to CPU 1
[19670705538] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb010b5a8
[19672242480] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned clock (PID=22)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19673739855] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=22 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369981696 RFLAGS_BEFORE=134 CR3_BEFORE=70615040 fs_base=0 gs_base=18446744071564586576
[19677161592] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/taskman
[19678057278] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19679802582] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19689169830] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[19693323375] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=210000 exec=false
[19700665776] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 23 (user task/process) assigned to CPU 2
[19703387814] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned taskman (PID=23)
[19705638381] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/fontd
[19706524398] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19708310127] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19733958453] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=226000 exec=false
[19739158362] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=22d000 exec=false
[19747268904] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 24 (user task/process) assigned to CPU 3
[19750472214] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned fontd (PID=24)
[19751356812] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010d510
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19753682025] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=24 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370113744 RFLAGS_BEFORE=130 CR3_BEFORE=70918144 fs_base=0 gs_base=18446744071564586640
[19757661264] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/blossom
[19758595032] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19759794120] [INFO] [fontd] [CPU3] FONTD: Starting Graph-Native Font Service v2 (Atlas-based IPC)
[19760697561] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19778363715] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=219000 exec=false
[19781402289] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=21d000 exec=false
[19783024470] [INFO] [fontd] [CPU3] FONTD: Service node created, req=15, resp=18
[19786670079] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[19788118317] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[19789273515] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[19790885433] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=225 pred=0 subj_lo=0
[19793388549] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 25 (user task/process) assigned to CPU 1
[19796287962] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned blossom (PID=25)
[19798105404] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/flytrap
[19798885953] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19800901890] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19808084439] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00f1068
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19810380876] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=25 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370198048 RFLAGS_BEFORE=130 CR3_BEFORE=71192576 fs_base=0 gs_base=18446744071564586576
[19817899299] [INFO] [blossom] [CPU1] BLOSSOM: Starting SVG Cache Service
[19819944441] [INFO] [blossom] [CPU1] BLOSSOM: Init UI pipeline...
[19824581964] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[19826311065] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[19827503157] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[19828663206] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[19829975385] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[19831291095] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[19832526714] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=229 pred=0 subj_lo=0
[19834056429] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=230 subj_lo=0
[19838286930] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=237000 exec=false
[19848373776] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=246000 exec=false
[19855627242] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 26 (user task/process) assigned to CPU 2
[19858333869] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned flytrap (PID=26)
[19861978917] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb0105370
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19863631689] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=20 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369849088 RFLAGS_BEFORE=134 CR3_BEFORE=70225920 fs_base=0 gs_base=18446744071564586608
[19868601357] [INFO] [nectar] [CPU2] NECTAR: Started.
[19870787343] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb010b5a8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19872613068] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[19874111301] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[19875266961] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[19876448328] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[19877538582] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[19878930126] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=23 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370047952 RFLAGS_BEFORE=130 CR3_BEFORE=70762496 fs_base=0 gs_base=18446744071564586608
[19882954113] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=231 pred=0 subj_lo=0
[19884450696] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[19886087265] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=232 subj_lo=0
[19890816528] [INFO] [taskman] [CPU2] TASKMAN: starting task manager
[19893406731] [INFO] [fontd] [CPU3] FONTD: Opened ASSET watch (handle=1247) for kind 'Asset'
[19894391517] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb010df80
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19896958587] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=26 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370266704 RFLAGS_BEFORE=134 CR3_BEFORE=71401472 fs_base=0 gs_base=18446744071564586608
[19901962080] [INFO] [fontd] [CPU3] FONTD: Service ready
[19903954158] [INFO] [flytrap] [CPU2] FLYTRAP: Starting unified content provider service...
[19905724410] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[19906849776] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[19907985240] [INFO] [flytrap] [CPU2] FLYTRAP: Service contract validated - graph-native asset watcher
[19909703286] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[19910781429] [INFO] [flytrap] [CPU2] FLYTRAP: Initializing Limine module content source...
[19912225146] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=234 subj_lo=0
[19921254606] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 3] Starting Compositor
[19935812424] [INFO] [sprout::pipelines] [CPU2] SPROUT: Writing bloom BS: drv_req=1, drv_resp=4, bristle_evt=12
[19943910591] [INFO] [virtio::device] [CPU2] VirtIO: common_bar=4 common_off=0x0 notify_bar=4 notify_off=0x3000 mult=4
[19955536161] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[19956667005] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[19958599089] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[19959580938] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[19960578000] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[19961760192] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=239 subj_lo=0
[19964125797] [INFO] [sprout::pipelines] [CPU2] SPROUT: Bloom handles via BS=1251 backend=VirtIO-GPU
[19965845031] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/bloom
[19966712931] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19968417414] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19971001875] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[19972142025] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[19973159646] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[19974262671] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=240 subj_lo=0
[19979676453] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[19980637215] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[19981749975] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[19982839932] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=241 subj_lo=0
[19988852136] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[19989813096] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[19990835469] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[19992102669] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=242 pred=0 subj_lo=0
[20013238608] [INFO] [blossom] [CPU1] BLOSSOM: UI pipeline ready
[20027551302] [INFO] [blossom] [CPU1] BLOSSOM: Service node created, req=19, resp=22
[20028662016] [INFO] [blossom] [CPU1] BLOSSOM: Service ready
[20106224391] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=2d4000 exec=false
[20123331261] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=2ea000 exec=false
[20131389531] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 27 (user task/process) assigned to CPU 3
[20133934986] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00f0fe8
[20135010852] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned bloom (PID=27)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x4e3
[20136266073] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=27 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370356576 RFLAGS_BEFORE=134 CR3_BEFORE=72052736 fs_base=0 gs_base=18446744071564586640
[20139232872] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/echo
[20140081500] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20141673915] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20142408462] [INFO] [bloom::logging] [CPU3] bloom: logging initialized
[20147076609] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=204000 exec=false
[20148527487] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[20156464317] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 28 (user task/process) assigned to CPU 1
[20158900674] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00f0fe8
[20160165366] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned echo (PID=28)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xe
[20162095041] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=28 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370422656 RFLAGS_BEFORE=130 CR3_BEFORE=73101312 fs_base=0 gs_base=18446744071564586576
[20166974817] [INFO] [sprout::pipelines] [CPU2] SPROUT: Compositor ready
[20168979303] [INFO] [echo] [CPU1] echo: starting up
[20171474697] [INFO] [bloom] [CPU3] bloom: [bloom] SIMD backend: SSE2 (x86_64)
[20172425130] [INFO] [echo] [CPU1] echo: ready for Bristle events (main loop using handle 14)
[20182156434] [INFO] [bloom] [CPU3] [bloom] EARLY boot args: bristle_evt=12 arg_req=1 arg_resp=4
[20184104457] [INFO] [bloom::compositor] [CPU3] bloom: map_from_bytespace start bs=1233
[20195028480] [INFO] [bloom::compositor] [CPU3] bloom: symbols loaded
[20197018083] [INFO] [bloom::compositor] [CPU3] bloom: loop iter 1
[20200740912] [INFO] [bloom::compositor] [CPU3] bloom: got width 1920
[20204839875] [INFO] [virtio::device] [CPU2] VirtIO: mapping common BAR4...
[20206660485] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR4 phys=0xc000004000 size=0x4000 -> virt=0x107f5000
[20209349259] [INFO] [virtio::device] [CPU2] VirtIO: common_cfg at 0x107f5000
[20211387702] [INFO] [virtio::device] [CPU2] VirtIO: notify_cfg at 0x107f8000
[20214401196] [INFO] [virtio::device] [CPU2] VirtIO: device_cfg = Some(276787200)
[20216400897] [INFO] [virtio::device] [CPU2] VirtIO: allocating DMA command buffer...
[20218951896] [INFO] [virtio::device] [CPU2] VirtIO: cmd_buf virt=0x107f9000 phys=0x4656000
[20220307668] [INFO] [virtio::device] [CPU2] VirtIO: device::new complete
[20223041190] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Device features 0x30bf8024
[20225333898] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: MAC 52:54:00:12:34:56
[20227030791] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Link UP
[20231096754] [INFO] [bloom::compositor] [CPU3] bloom: got properties w=1920 h=1080 s=7680 f=1
[20232674748] [INFO] [bloom::compositor] [CPU3] bloom: properties OK, building from config...
[20234513970] [INFO] [bloom::compositor] [CPU3] bloom: compositor bytespace 1233 (1920x1080 stride=7680 format=1)
[20237326923] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX queue 0 setup (size=32)
[20248461816] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX queue 1 setup (size=32)
[20249334963] [INFO] [bloom::compositor] [CPU3] bloom: display backend: VirtIO-GPU
[20254907541] [INFO] [bloom::compositor] [CPU3] bloom: mapped size=8294400 (source=bytespace_info)
[20257841802] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Allocated RX pool (16 pages, 32 buffers)
[20260567503] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Allocated TX buffer (1 page)
[20262770517] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX queue filled with 32 buffers
[20264986632] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: DRIVER_OK set, device is live
[20266069527] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[20267251950] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Driver initialized successfully
[20268342765] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[20269236999] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Driver initialized successfully
[20270583663] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: MAC 52:54:00:12:34:56
[20272496376] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Waiting for link...
[20273868681] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Link is UP
[20279198379] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created port (write=23, read=24)
[20284067925] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created RX port (write=25, read=26)
[20291007792] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 4] Proof of life (Beeper)
[20292402438] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawning beeper...
[20293955814] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/beeper
[20294741841] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20295951225] [INFO] [bloom::present] [CPU3] bloom: driver REGISTER (kind=2 caps=0x3)
[20296851729] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20298834864] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: received MSG_HELLO
[20299733817] [INFO] [bloom::present] [CPU3] display: negotiated proto 1.0 caps=DIRTY_RECTS|FULLFRAME max_rects=32
[20301714510] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[20303605608] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[20312085156] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 29 (user task/process) assigned to CPU 0
[20315136237] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned beeper (PID=29)
[20317525998] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 5] Starting Discovered User Apps
[20319084555] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/clock'
[20320720332] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/clock
[20321511144] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20323719306] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20334073386] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[20339477103] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20e000 exec=false
[20340762189] [INFO] [bloom] [CPU3] [bloom] Requesting initial driver buffer...
[20350293876] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 30 (user task/process) assigned to CPU 3
[20354660172] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=30)
[20356459926] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/hello_std'
[20357813454] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([235, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[20359124181] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/hello_std
[20360329077] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20362458105] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20366820111] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=202000 exec=false
[20368547958] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=203000 exec=false
[20369780145] [INFO] [bloom] [CPU3] [bloom] ACQUIRED RETURNED!
[20372037477] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00f1548
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[20373854721] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=30 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370706672 RFLAGS_BEFORE=134 CR3_BEFORE=74002432 fs_base=0 gs_base=18446744071564586640
[20380465941] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 31 (user task/process) assigned to CPU 1
[20384526360] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=31)
[20386396866] [INFO] [sprout::supervisor] [CPU2] SPROUT: Startup complete. Entering monitor loop.
[20389364622] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00f1548
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[20391910605] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=31 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370772208 RFLAGS_BEFORE=134 CR3_BEFORE=74149888 fs_base=0 gs_base=18446744071564586576
[20396792295] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created service node ThingId([235, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[20399680587] [ERROR] [kernel::trap] [CPU1] user_invalid_opcode rip=0x0000000000201420 err=0x0000 rsp=0x00000000007fff08 cs=0x2b kind=6
[20413649190] [INFO] [bloom] [CPU3] [bloom] ACQUIRED initial driver buffer: 0x10ffc000 (bs_id=ThingId([214, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]))
[20415975756] [INFO] [bloom] [CPU3] bloom: creating surface...
[20417442210] [INFO] [bloom] [CPU3] bloom: surface created!
[20418816528] [INFO] [bloom] [CPU3] bloom: finding UI CROWN...
[20426867439] [INFO] [bloom] [CPU3] bloom: creating new UI CROWN
[20438166111] [INFO] [stem::ui] [CPU3] UiBuilder: created root 1261
[20439162018] [INFO] [bloom] [CPU3] bloom: UI CROWN initialized!
[20440593921] [INFO] [flytrap] [CPU2] FLYTRAP: Created Limine ContentSource node
[20441749284] [INFO] [flytrap] [CPU2] FLYTRAP: Performing initial boot module scan...
[20444348529] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Published service - TX port=23, RX port=26
[20454064884] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 32 (user thread) assigned to CPU 3
[20456895657] [INFO] [bloom] [CPU3] bloom: spawned asset watcher (tid=32)
[20458057851] [INFO] [bloom::frame_loop] [CPU3] bloom: running (fps_target=60)
[20458874733] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[20460140019] [INFO] [bloom] [CPU3] [bloom] bristle_evt_handle = 12 (legacy was 12)
[20461097085] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[20464242051] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([235, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[20468246667] [INFO] [fetchd] [CPU3] FETCHD: Found UI Root: 1261
T:0270 [20473435719] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] asset_watcher_entry: spawning sub-loaders
[20481647538] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 33 (user thread) assigned to CPU 3
[20489757981] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 34 (user thread) assigned to CPU 3
[20497315674] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 35 (user thread) assigned to CPU 3
T:07D0 T:0640 T:F0B0 [20532131301] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] wallpaper loader: loading flower.bmp
[20538611478] [INFO] [taskman] [CPU2] TASKMAN: Found UI Crown (attempt 3)
[20541275469] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 36 (user thread) assigned to CPU 3
[20543689617] [DEBUG] [bloom::asset] [CPU3] [asset_bank] worker spawned tid=36 (priority=2)
[20545844220] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] cursor loader: loading default cursor
[20548129239] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] icon loader started
[20553993207] [INFO] [bloom] [CPU3] bloom: GPU composition not supported (or Virgl disabled) - using CPU composition mode
[20555519391] [INFO] [bloom] [CPU3] bloom: calling intern for FONT_GLYPH
[20559388146] [INFO] [netd] [CPU3] NETD: Driver TX port=23, RX port=26, link_up=true, mtu=1500
[20561680623] [INFO] [netd] [CPU3] NETD: Connected to driver - MAC 52:54:00:12:34:56
[20565384873] [INFO] [netd] [CPU3] NETD: Created socket API port (write=27, read=28)
T:1220 [20574631737] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/sprout' (raw, 111120 bytes, hash=a121b43754098a0b)
[20579062383] [INFO] [bloom] [CPU3] bloom: intern returned FONT_GLYPH=274
[20580196428] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for FONT_GLYPH
[20581375848] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[20582295228] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[20583387396] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[20584693305] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=274 subj_lo=0
[20590704882] [DEBUG] [bloom::asset] [CPU3] [asset_bank] worker started (priority bump)
[20592904200] [DEBUG] [bloom::asset] [CPU3] [asset_bank] load_wallpaper_immediate: flower.bmp
[20606578179] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1272)
[20607765915] [INFO] [bloom] [CPU3] bloom: calling intern for UI_WINDOW
[20619833586] [INFO] [bloom] [CPU3] bloom: intern returned UI_WINDOW=242
[20621124975] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for UI_WINDOW
[20622189753] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[20623216284] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[20624551002] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[20626228491] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=242 pred=0 subj_lo=0
[20643996285] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1275)
[20645237943] [INFO] [bloom] [CPU3] bloom: calling intern for UI_PAINT_GEN
[20661938715] [INFO] [bloom] [CPU3] bloom: intern returned UI_PAINT_GEN=279
[20664037977] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for UI_PAINT_GEN
[20665384311] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[20666398467] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[20667710811] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[20669157300] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=279 subj_lo=0
[20686885758] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1278)
[20688709272] [INFO] [bloom] [CPU3] [bloom] Starting UI loop immediately (not waiting for fonts)
[20691634854] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=27, our_write=31, our_read=32)
[20693708574] [INFO] [anther] [CPU1] anther: Connected to network stack
[20697836016] [INFO] [netd] [CPU3] NETD: Published initial stack node ThingId([246, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) to graph
[20766899769] [INFO] [netd] [CPU3] NETD: Starting DHCP...
[20768724141] [INFO] [netd::dhcp] [CPU3] DHCP: Starting discovery...
[20776279623] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 304 bytes - IPv4 other
[20808369780] [INFO] [netd::dhcp] [CPU3] DHCP: Deconfigured
[20811569691] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 304 bytes
[20818263081] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=0 len=600
[20844350505] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 590 byte frame (594 encoded) to netd rx_port=25
[20846842071] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (594 bytes sent)
[20853248988] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/bristle' (raw, 41456 bytes, hash=8aec727fb8928c70)
[20879786829] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 590 bytes
[20886967332] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 316 bytes - IPv4 other
[20890092036] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 316 bytes
[20895260232] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=1 len=600
[20897784171] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 590 byte frame (594 encoded) to netd rx_port=25
[20901668733] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (594 bytes sent)
[20913773364] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 590 bytes
[20917700001] [INFO] [netd::dhcp] [CPU3] DHCP: Configuration received
[20919899682] [INFO] [netd] [CPU3] NETD: DHCP complete - IP: 10.0.2.15, Gateway: 10.0.2.2, DNS: 10.0.2.3
[20968213332] [INFO] [netd] [CPU3] NETD: DHCP configured — IP: 10.0.2.15, GW: 10.0.2.2, DNS: 10.0.2.3
[20971403838] [INFO] [netd] [CPU3] NETD: Network stack ready, entering service loop
[20976546360] [INFO] [netd::socket_api] [CPU3] SOCKET_API: UDP_BIND on port 5353
[20979035220] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Bound UDP on port 5353, handle=1
[20981650470] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_LISTEN on port 80
[20984089896] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Listening on port 80, handle=2
[20991491598] [INFO] [anther] [CPU1] anther: Listening on port 80 (handle=2)
[21034070376] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Joining multicast group 224.0.0.251
[21036180990] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 42 bytes - IPv4 other
[21037936986] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 42 bytes
[21076062579] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/rtc_cmos' (raw, 29248 bytes, hash=562b9fa5b03c7a45)
[21136695591] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping bytespace 741 (8386614 bytes) for 'flower.bmp'
[21158053026] [DEBUG] [bloom::asset] [CPU3] [asset_bank] decoding BMP for 'flower.bmp'...
[21231920226] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/clock' (raw, 61968 bytes, hash=cae77ff7f85d3f2f)
[21341053503] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=4000 nodes=663 watches=13 history=1024 journal=1024 symbols=309 drops=0
[21397328799] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/taskman' (raw, 70160 bytes, hash=d3dcbbde76a4f136)
[21532163697] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/ps2_kbd' (raw, 25072 bytes, hash=e5e306bcbc479ba0)
[21690178125] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/echo' (raw, 25072 bytes, hash=015ec546bc28642e)
[21951754836] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/bloom' (raw, 965128 bytes, hash=f2305eb88ad8f119)
[22112510178] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/ps2_mouse' (raw, 29168 bytes, hash=87af446dc7a3dc67)
[22276029402] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/display_bootfb' (raw, 29248 bytes, hash=fd9c527f4c1617be)
[22471335579] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/display_virtio_gpu' (raw, 53864 bytes, hash=bc0074690049f9cd)
[22482727113] [DEBUG] [bloom::asset] [CPU3] [asset_bank] BMP decoded: 2048x1365 for 'flower.bmp'
[22667448408] [INFO] [taskman] [CPU2] TASKMAN: Window created, found 1 initial tasks
[22673313993] [WARN] [WARN] [CPU2] root_watch_open: WATCH_START_LATEST is deprecated; use start_seq=0 + watch_drain.
[22675800675] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[22677414771] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[22679380944] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=18446744073709551615
[22681604022] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=230 subj_lo=0
[22696986345] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[22698622716] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[22700562093] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=18446744073709551615
[22702824144] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=337 pred=0 subj_lo=0
[22709139024] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/fontd' (raw, 188944 bytes, hash=55b938b69928e656)
[22891048884] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/blossom' (raw, 123408 bytes, hash=99095735adb015b8)
[23017007574] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=5000 nodes=714 watches=15 history=1024 journal=1024 symbols=339 drops=0
[23097836619] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/flytrap' (raw, 291352 bytes, hash=bf23e61b1474078c)
[23272905810] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/virtio_netd' (raw, 49680 bytes, hash=e044c79e0fe2b998)
[23459532855] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/rtl8168d' (raw, 45584 bytes, hash=a6a9e038341d5a28)
[23675230722] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/netd' (raw, 147984 bytes, hash=fec1d29ef0678173)
[23881483758] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/fetchd' (raw, 41488 bytes, hash=4e0f74e1bbfcc0ce)
[23909639688] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_wallpaper (pending): 2048x1365
[23912237778] [DEBUG] [bloom::asset] [CPU3] [asset_bank] load_cursor_immediate: /assets/cursors/future/default.svg
[24099101532] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[24100234059] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[24101418891] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[24102756348] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=348 pred=0 subj_lo=0
[24214648953] [INFO] [bloom::present] [CPU3] display: full-frame damage, using full-frame present
[24260686890] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/anther' (raw, 586272 bytes, hash=c1e9a7b90a789e5c)
[24346979349] [INFO] [fetchd] [CPU3] FETCHD: Entering main loop, watching for network stack...
[24359032632] [INFO] [bloom] [CPU3] [CONTRACT] [bloom] First frame rendered
[24376816035] [DEBUG] [bloom::reclaimer] [CPU3] [reclaimer] +11182080 bytes (total: 11182080)
[24379362018] [INFO] [bloom::asset] [CPU3] [asset_bank] promoting wallpaper 'flower.bmp' to gen=1 (11182080b)
[24389232582] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[24391002768] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[24392822289] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[24394751832] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=349 pred=0 subj_lo=0
[24404747532] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[24409466598] [INFO] [fetchd] [CPU3] FETCHD: Got IP address: 10.0.2.15
[24503519139] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[24574839366] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/fortune' (raw, 61968 bytes, hash=543c2140ff247374)
[24584815992] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping bytespace 791 (3051 bytes)
[24587293335] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[24603383673] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapped to 0x122cd000
[24604863294] [DEBUG] [bloom::asset] [CPU3] [asset_bank] detected SVG format
[24606263121] [DEBUG] [bloom::asset] [CPU3] [asset_bank] parsing SVG cursor from /assets/cursors/future/default.svg
[24670070304] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=27 flat_sq=0.003125 edges=36
[24679802895] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=10 flat_sq=0.003125 edges=19
[24684961521] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=60 flat_sq=0.003125 edges=45
[24687952872] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=10 flat_sq=0.003125 edges=32
[24691116450] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=95 flat_sq=0.003125 edges=72
[24696756876] [DEBUG] [bloom::asset] [CPU3] [asset_bank] SUCCESS: SVG cursor rasterized 64x64
[24719889216] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_cursor (pending)
[24721499451] [DEBUG] [bloom::asset] [CPU3] [asset_bank] cursor: Static frame 64x64 hotspot (12, 8)
[24723235977] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping font bytespace 771 (569208 bytes)
[24742251732] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapped at 0x122d1000
[24743686473] [DEBUG] [bloom::asset] [CPU3] [asset_bank] parsing font '/assets/fonts/NotoSans-Regular.ttf'...
[24824129385] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/photosynthesis' (raw, 12648 bytes, hash=a0b92df72a4fb68a)
[25078011453] [ERROR] [kernel::trap] [CPU3] user_page_fault va=0x00000000122ee96a rip=0x00000000002a4a87 err=0x0004 present=0 user=1 write=0 instr_fetch=0
[25086289305] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/ahci_disk' (raw, 45664 bytes, hash=89d4c04e59e9cee2)
[25135371624] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[25167599688] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[25168897875] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[25170301695] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[25171681524] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=350 pred=0 subj_lo=0
[25233163791] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=6000 nodes=807 watches=18 history=1024 journal=1024 symbols=363 drops=0
[25239060198] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[25240306773] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[25241504475] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[25242785799] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=351 pred=0 subj_lo=0
[25255497696] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[25365426207] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[25457799576] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/iso9660d' (raw, 53856 bytes, hash=e0b08365f733f5f1)
[25483083120] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[25572893115] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[25669091613] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[25799904570] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[25863446433] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/virtio_sound' (raw, 49648 bytes, hash=cc70fc86a6ccc18f)
[26205570336] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/hdaudio' (raw, 37360 bytes, hash=efe87e268a526308)
[26495171868] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/pci_stubd' (raw, 29168 bytes, hash=cd275cc95b190f74)
[26816642556] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/beeper' (raw, 33296 bytes, hash=3662850ee0db2f79)
[27126562221] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/nectar' (raw, 176656 bytes, hash=9e2e57ba5e34944a)
[27374070801] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[27444472869] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/hello_std' (raw, 16880 bytes, hash=bcb44329b85efc54)
[27492201363] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=7000 nodes=870 watches=19 history=1024 journal=1024 symbols=364 drops=0
[27565625010] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[27637112712] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[27663652203] [DEBUG] [bloom::reclaimer] [CPU3] [reclaimer] +16384 bytes (total: 11198464)
[27666795849] [INFO] [bloom::asset] [CPU3] [asset_bank] promoting cursor to gen=2 (16384b)
[27756507207] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[27855052929] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/stdio_demo' (raw, 16880 bytes, hash=c2c5f3a4bb2c1649)
[27874160193] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[27957990720] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[28063276164] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[28169588898] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[28247895159] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/telnetd' (raw, 57872 bytes, hash=ada0370ab1250e73)
[28711604823] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/wallpapers/clouds.bmp' (image, 786486 bytes, hash=6302b2cf74732177)
[29648302167] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=8000 nodes=902 watches=19 history=1024 journal=1024 symbols=365 drops=0
[29701874037] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/wallpapers/flower.bmp' (image, 8386614 bytes, hash=4e15e281a6ce0335)
[29872393287] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=2429 ops=1 watches=19
[30161596641] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/wallpapers/leather.bmp' (image, 1179702 bytes, hash=97469f321dc8d354)
[30942798612] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/wallpapers/linen.bmp' (image, 4718646 bytes, hash=713fedbc593ffc21)
[31417867767] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/fonts/NotoSans-Regular.ttf' (font, 569208 bytes, hash=fb7c8aeaec385cb8)
[31488179877] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=9000 nodes=936 watches=19 history=1024 journal=1024 symbols=365 drops=0
[31728436938] [INFO] [flytrap] [CPU2] FLYTRAP: Font debug - name='/assets/fonts/NotoSans-Regular.ttf' kind='font' guess=Some("application/font-sfnt") first4=[00, 01, 00, 00]
[31868519001] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/themes/genie_circles.wasm' (raw, 2824 bytes, hash=f4422f600444a873)
[32108589975] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[32196007437] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[32288533695] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/assets/cursors/future/default.svg' (svg, 3051 bytes, hash=94bd3615327459d1)
[33368930727] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[33568220433] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=10000 nodes=1003 watches=19 history=1024 journal=1024 symbols=407 drops=0
[34498569072] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[34594966263] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[35103696936] [INFO] [flytrap] [CPU2] FLYTRAP: Parsed SVG '/assets/cursors/future/default.svg' as XML tree (18 elements, 46 attrs)
[35133676941] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[35159234352] [INFO] [bloom::cursor_rasterizer] [CPU3] [cursor_rasterizer] rasterizing cursor snapshot gen=2
[35223946593] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/locale.conf' (raw, 85 bytes, hash=47898ce45a9f4c24)
[35264272197] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[35347300461] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[35446309008] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[35525510064] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[35596497420] [INFO] [flytrap] [CPU2] FLYTRAP: Limine boot module scan complete (indexed 39 assets)
[35599272456] [INFO] [flytrap] [CPU2] FLYTRAP: Seeding system assets (reactive)...
[35628040800] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[35641274493] [INFO] [flytrap] [CPU2] FLYTRAP: Linked ui.Crown to svc.Root
[35778745134] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[36172258188] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=11000 nodes=1108 watches=19 history=1024 journal=1024 symbols=449 drops=0
[36233720061] [INFO] [flytrap] [CPU2] FLYTRAP: Seeding desktop wallpaper 'leather.bmp'
[36282519603] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[36284242698] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[36285398952] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=0
[36286707930] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=43 pred=0 subj_lo=0
[36299193381] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[36300349371] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[36301698972] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=0
[36302981484] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=235 pred=0 subj_lo=0
[36310906929] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[36312108492] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[36313355595] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=0
[36314675034] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=451 pred=0 subj_lo=0
[36322447425] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[36323551275] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[36324786036] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=0
[36326228499] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=450 pred=0 subj_lo=0
[36336806649] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[36337976136] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[36339262938] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=0
[36340488888] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=94 pred=0 subj_lo=0
[36348663813] [INFO] [flytrap] [CPU2] FLYTRAP: Continuous asset watcher loop active. Watching for asset changes...
[36441744471] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[36530738904] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[38138047629] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=12000 nodes=1145 watches=24 history=1024 journal=1024 symbols=452 drops=0
[38723371038] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[39415620420] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[39523929027] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[39633323829] [INFO] [fontd] [CPU3] FONTD: Auto-importing font asset ThingId([44, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[40054503357] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=13000 nodes=1184 watches=24 history=1024 journal=1024 symbols=453 drops=0
[40554367623] [INFO] [fontd] [CPU3] FONTD: Auto-imported font 'Noto Sans' style 'Regular' from asset ThingId([44, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[40556830908] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (1 fonts)
[41751633924] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=14000 nodes=1222 watches=24 history=1024 journal=1024 symbols=455 drops=0
[43565710188] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=15000 nodes=1248 watches=24 history=1024 journal=1024 symbols=455 drops=0
[43735949499] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=3453 ops=1 watches=24
[45138835467] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=16000 nodes=1274 watches=24 history=1024 journal=1024 symbols=455 drops=0
[45372426792] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[46999643295] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=17000 nodes=1305 watches=24 history=1024 journal=1024 symbols=455 drops=0
[49246631016] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=18000 nodes=1329 watches=24 history=1024 journal=1024 symbols=455 drops=0
[51774849522] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=19000 nodes=1359 watches=24 history=1024 journal=1024 symbols=455 drops=0
[54791927322] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=20000 nodes=1392 watches=24 history=1024 journal=1024 symbols=455 drops=0
[57912329673] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=21000 nodes=1432 watches=24 history=1024 journal=1024 symbols=455 drops=0
[59991751050] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=22000 nodes=1456 watches=24 history=1024 journal=1024 symbols=455 drops=0
[62984208012] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=23000 nodes=1489 watches=24 history=1024 journal=1024 symbols=455 drops=0
[66090308031] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=24000 nodes=1521 watches=24 history=1024 journal=1024 symbols=455 drops=0
[68640735636] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=25000 nodes=1549 watches=24 history=1024 journal=1024 symbols=455 drops=0
[71144978223] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=26000 nodes=1586 watches=24 history=1024 journal=1024 symbols=455 drops=0
[74182817214] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=27000 nodes=1614 watches=24 history=1024 journal=1024 symbols=455 drops=0
[76921683795] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=28000 nodes=1638 watches=24 history=1024 journal=1024 symbols=455 drops=0
[78330623679] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=4477 ops=1 watches=24
[79853962452] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=29000 nodes=1671 watches=24 history=1024 journal=1024 symbols=455 drops=0
[82610386419] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=30000 nodes=1695 watches=24 history=1024 journal=1024 symbols=455 drops=0
[85607448663] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=31000 nodes=1729 watches=24 history=1024 journal=1024 symbols=455 drops=0
[87945374220] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=32000 nodes=1755 watches=24 history=1024 journal=1024 symbols=455 drops=0
[90082122801] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=33000 nodes=1790 watches=24 history=1024 journal=1024 symbols=455 drops=0
[91860402678] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=34000 nodes=1814 watches=24 history=1024 journal=1024 symbols=455 drops=0
[93983864898] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=35000 nodes=1842 watches=24 history=1024 journal=1024 symbols=455 drops=0
[96070099191] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=36000 nodes=1873 watches=24 history=1024 journal=1024 symbols=455 drops=0
[98100907476] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=37000 nodes=1901 watches=24 history=1024 journal=1024 symbols=455 drops=0
[100004356320] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=38000 nodes=1931 watches=24 history=1024 journal=1024 symbols=455 drops=0
[102816564312] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=39000 nodes=1964 watches=24 history=1024 journal=1024 symbols=455 drops=0
[105362363304] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=40000 nodes=1988 watches=24 history=1024 journal=1024 symbols=455 drops=0
[107399291274] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=41000 nodes=2016 watches=24 history=1024 journal=1024 symbols=455 drops=0
[109476823434] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=42000 nodes=2040 watches=24 history=1024 journal=1024 symbols=455 drops=0
[110253785103] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=5501 ops=1 watches=24
[111519032988] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=43000 nodes=2073 watches=24 history=1024 journal=1024 symbols=455 drops=0
[113593314450] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=44000 nodes=2099 watches=24 history=1024 journal=1024 symbols=455 drops=0
[115902285708] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=45000 nodes=2135 watches=24 history=1024 journal=1024 symbols=455 drops=0
[118118606892] [INFO] [ke
```
</details>
