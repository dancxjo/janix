# ❌ Scenario: Task Manager displays the process list

> Last run: 2026-04-05 17:45:26

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is started | ✅ | 0ms | - - - |
| 2 | When I wait for the system to reach ready state | ✅ | 8713ms | - - - |
| 3 | Then I should see a message in the serial output that says "TASKMAN: starting task manager" within 300s | ✅ | 1829ms | - [📜](./03/serial.log) - |
| 4 | And the serial output should contain "TASKMAN: Window created" | ✅ | 1837ms | - [📜](./04/serial.log) - |
| 5 | And I should see a rectangle at 8, 30 with size 500x400 and color "#F0F0F0" | ❌ | 1023ms | - [📜](./05/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[18991207422] [INFO] [kernel] [CPU0] BOOTFB: width=1920 height=1080 pitch=7680 bpp=32 format=Bgrx8888
[18998952060] [CONTRACT] [kernel] [CPU0] thing-os kernel starting...
[19005665448] [INFO] [bran::requests] [CPU0] Limine: Found 39 boot modules
[19009173744] [INFO] [bran::requests] [CPU0]   [0] /boot/sprout (cmdline='init') size=111120
[19011160542] [INFO] [bran::requests] [CPU0]   [1] /boot/bristle (cmdline='init') size=41456
[19012182651] [INFO] [bran::requests] [CPU0]   [2] /boot/rtc_cmos (cmdline='init') size=29248
[19013263137] [INFO] [bran::requests] [CPU0]   [3] /boot/clock (cmdline='') size=61968
[19014189183] [INFO] [bran::requests] [CPU0]   [4] /boot/taskman (cmdline='') size=70160
[19015115823] [INFO] [bran::requests] [CPU0]   [5] /boot/ps2_kbd (cmdline='init') size=25072
[19016121960] [INFO] [bran::requests] [CPU0]   [6] /boot/echo (cmdline='init') size=25072
[19017083646] [INFO] [bran::requests] [CPU0]   [7] /boot/bloom (cmdline='init') size=965128
[19018092621] [INFO] [bran::requests] [CPU0]   [8] /boot/ps2_mouse (cmdline='init') size=29168
[19019279235] [INFO] [bran::requests] [CPU0]   [9] /boot/display_bootfb (cmdline='init') size=29248
[19020371106] [INFO] [bran::requests] [CPU0]   [10] /boot/display_virtio_gpu (cmdline='init') size=53864
[19021491687] [INFO] [bran::requests] [CPU0]   [11] /boot/fontd (cmdline='init') size=188944
[19022519703] [INFO] [bran::requests] [CPU0]   [12] /boot/blossom (cmdline='init') size=123408
[19023501948] [INFO] [bran::requests] [CPU0]   [13] /boot/flytrap (cmdline='') size=291352
[19024423506] [INFO] [bran::requests] [CPU0]   [14] /boot/virtio_netd (cmdline='') size=49680
[19025390274] [INFO] [bran::requests] [CPU0]   [15] /boot/rtl8168d (cmdline='') size=45584
[19026375687] [INFO] [bran::requests] [CPU0]   [16] /boot/netd (cmdline='') size=147984
[19027280844] [INFO] [bran::requests] [CPU0]   [17] /boot/fetchd (cmdline='') size=41488
[19027911705] [INFO] [bran::requests] [CPU0]   [18] /boot/anther (cmdline='') size=586272
[19028543721] [INFO] [bran::requests] [CPU0]   [19] /boot/fortune (cmdline='') size=61968
[19029205899] [INFO] [bran::requests] [CPU0]   [20] /boot/photosynthesis (cmdline='') size=12648
[19029912792] [INFO] [bran::requests] [CPU0]   [21] /boot/ahci_disk (cmdline='') size=45664
[19030907676] [INFO] [bran::requests] [CPU0]   [22] /boot/iso9660d (cmdline='') size=53856
[19031917608] [INFO] [bran::requests] [CPU0]   [23] /boot/virtio_sound (cmdline='init') size=49648
[19033001823] [INFO] [bran::requests] [CPU0]   [24] /boot/hdaudio (cmdline='') size=37360
[19033948692] [INFO] [bran::requests] [CPU0]   [25] /boot/pci_stubd (cmdline='') size=29168
[19034941266] [INFO] [bran::requests] [CPU0]   [26] /boot/beeper (cmdline='init') size=33296
[19035935028] [INFO] [bran::requests] [CPU0]   [27] /boot/nectar (cmdline='init') size=176656
[19036971228] [INFO] [bran::requests] [CPU0]   [28] /boot/hello_std (cmdline='') size=16880
[19038009177] [INFO] [bran::requests] [CPU0]   [29] /boot/stdio_demo (cmdline='') size=16880
[19039045971] [INFO] [bran::requests] [CPU0]   [30] /boot/telnetd (cmdline='') size=57872
[19039996998] [INFO] [bran::requests] [CPU0]   [31] /assets/wallpapers/clouds.bmp (cmdline='') size=786486
[19041128040] [INFO] [bran::requests] [CPU0]   [32] /assets/wallpapers/flower.bmp (cmdline='') size=8386614
[19042310133] [INFO] [bran::requests] [CPU0]   [33] /assets/wallpapers/leather.bmp (cmdline='') size=1179702
[19043507340] [INFO] [bran::requests] [CPU0]   [34] /assets/wallpapers/linen.bmp (cmdline='') size=4718646
[19044571722] [INFO] [bran::requests] [CPU0]   [35] /assets/fonts/NotoSans-Regular.ttf (cmdline='') size=569208
[19045372929] [INFO] [bran::requests] [CPU0]   [36] /assets/themes/genie_circles.wasm (cmdline='') size=2824
[19046126088] [INFO] [bran::requests] [CPU0]   [37] /assets/cursors/future/default.svg (cmdline='') size=3051
[19047001908] [INFO] [bran::requests] [CPU0]   [38] /boot/locale.conf (cmdline='') size=85
[19049191293] [INFO] [kernel::memory] [CPU0] Memory map has 64 entries
[19051501689] [INFO] [kernel::memory] [CPU0]   [0] 0x0 - 0x87000 (Usable)
[19052363715] [INFO] [kernel::memory] [CPU0]   [1] 0x87000 - 0x88000 (Reserved)
[19052943822] [INFO] [kernel::memory] [CPU0]   [2] 0x88000 - 0xa0000 (Usable)
[19053485517] [INFO] [kernel::memory] [CPU0]   [3] 0x100000 - 0x800000 (Usable)
[19054038333] [INFO] [kernel::memory] [CPU0]   [4] 0x800000 - 0x808000 (Other)
[19054878975] [INFO] [kernel::memory] [CPU0]   [5] 0x808000 - 0x80b000 (Usable)
[19055464758] [INFO] [kernel::memory] [CPU0]   [6] 0x80b000 - 0x80c000 (Other)
[19056015066] [INFO] [kernel::memory] [CPU0]   [7] 0x80c000 - 0x811000 (Usable)
[19056570456] [INFO] [kernel::memory] [CPU0]   [8] 0x811000 - 0x900000 (Other)
[19057119081] [INFO] [kernel::memory] [CPU0]   [9] 0x900000 - 0x1780000 (Reserved)
[19057974903] [INFO] [kernel::memory] [CPU0]   [10] 0x1780000 - 0x7797d000 (Usable)
[19058873856] [INFO] [kernel::memory] [CPU0]   [11] 0x7797d000 - 0x779e1000 (Reserved)
[19059790992] [INFO] [kernel::memory] [CPU0]   [12] 0x779e1000 - 0x779e2000 (Other)
[19060685094] [INFO] [kernel::memory] [CPU0]   [13] 0x779e2000 - 0x779e3000 (Reserved)
[19061607411] [INFO] [kernel::memory] [CPU0]   [14] 0x779e3000 - 0x779e4000 (Other)
[19062492537] [INFO] [kernel::memory] [CPU0]   [15] 0x779e4000 - 0x779e5000 (Reserved)
[19063402842] [INFO] [kernel::memory] [CPU0]   [16] 0x779e5000 - 0x779e6000 (Other)
[19064287011] [INFO] [kernel::memory] [CPU0]   [17] 0x779e6000 - 0x779e7000 (Reserved)
[19065209889] [INFO] [kernel::memory] [CPU0]   [18] 0x779e7000 - 0x77a72000 (Other)
[19066123230] [INFO] [kernel::memory] [CPU0]   [19] 0x77a72000 - 0x77a73000 (Reserved)
[19067035845] [INFO] [kernel::memory] [CPU0]   [20] 0x77a73000 - 0x77ef4000 (Other)
[19067907705] [INFO] [kernel::memory] [CPU0]   [21] 0x77ef4000 - 0x77ef5000 (Reserved)
[19068855399] [INFO] [kernel::memory] [CPU0]   [22] 0x77ef5000 - 0x78016000 (Other)
[19069739733] [INFO] [kernel::memory] [CPU0]   [23] 0x78016000 - 0x78017000 (Reserved)
[19070645253] [INFO] [kernel::memory] [CPU0]   [24] 0x78017000 - 0x78817000 (Other)
[19071532326] [INFO] [kernel::memory] [CPU0]   [25] 0x78817000 - 0x78818000 (Reserved)
[19072458042] [INFO] [kernel::memory] [CPU0]   [26] 0x78818000 - 0x788d9000 (Other)
[19073347722] [INFO] [kernel::memory] [CPU0]   [27] 0x788d9000 - 0x788da000 (Reserved)
[19074244926] [INFO] [kernel::memory] [CPU0]   [28] 0x788da000 - 0x788e9000 (Other)
[19075130481] [INFO] [kernel::memory] [CPU0]   [29] 0x788e9000 - 0x788ea000 (Reserved)
[19076018643] [INFO] [kernel::memory] [CPU0]   [30] 0x788ea000 - 0x788ef000 (Other)
[19076857140] [INFO] [kernel::memory] [CPU0]   [31] 0x788ef000 - 0x788f0000 (Reserved)
[19077715305] [INFO] [kernel::memory] [CPU0]   [32] 0x788f0000 - 0x788f5000 (Other)
[19078590630] [INFO] [kernel::memory] [CPU0]   [33] 0x788f5000 - 0x788f6000 (Reserved)
[19079531988] [INFO] [kernel::memory] [CPU0]   [34] 0x788f6000 - 0x78922000 (Other)
[19080450675] [INFO] [kernel::memory] [CPU0]   [35] 0x78922000 - 0x78924000 (Reserved)
[19081383651] [INFO] [kernel::memory] [CPU0]   [36] 0x78924000 - 0x7892d000 (Other)
[19082280162] [INFO] [kernel::memory] [CPU0]   [37] 0x7892d000 - 0x7892f000 (Reserved)
[19083222708] [INFO] [kernel::memory] [CPU0]   [38] 0x7892f000 - 0x78937000 (Other)
[19084119285] [INFO] [kernel::memory] [CPU0]   [39] 0x78937000 - 0x78938000 (Reserved)
[19085034903] [INFO] [kernel::memory] [CPU0]   [40] 0x78938000 - 0x78942000 (Other)
[19085925045] [INFO] [kernel::memory] [CPU0]   [41] 0x78942000 - 0x78943000 (Reserved)
[19086915804] [INFO] [kernel::memory] [CPU0]   [42] 0x78943000 - 0x78950000 (Other)
[19087769712] [INFO] [kernel::memory] [CPU0]   [43] 0x78950000 - 0x78952000 (Reserved)
[19088704272] [INFO] [kernel::memory] [CPU0]   [44] 0x78952000 - 0x78960000 (Other)
[19089605172] [INFO] [kernel::memory] [CPU0]   [45] 0x78960000 - 0x78961000 (Reserved)
[19090501650] [INFO] [kernel::memory] [CPU0]   [46] 0x78961000 - 0x7896d000 (Other)
[19091312724] [INFO] [kernel::memory] [CPU0]   [47] 0x7896d000 - 0x7896e000 (Reserved)
[19091918670] [INFO] [kernel::memory] [CPU0]   [48] 0x7896e000 - 0x78972000 (Other)
[19092502275] [INFO] [kernel::memory] [CPU0]   [49] 0x78972000 - 0x78973000 (Reserved)
[19093101621] [INFO] [kernel::memory] [CPU0]   [50] 0x78973000 - 0x78983000 (Other)
[19093681398] [INFO] [kernel::memory] [CPU0]   [51] 0x78983000 - 0x78984000 (Reserved)
[19094277576] [INFO] [kernel::memory] [CPU0]   [52] 0x78984000 - 0x78a14000 (Other)
[19094889561] [INFO] [kernel::memory] [CPU0]   [53] 0x78a14000 - 0x78a15000 (Reserved)
[19095488742] [INFO] [kernel::memory] [CPU0]   [54] 0x78a15000 - 0x78a20000 (Other)
[19096249953] [INFO] [kernel::memory] [CPU0]   [55] 0x78a20000 - 0x78a21000 (Reserved)
[19097195832] [INFO] [kernel::memory] [CPU0]   [56] 0x78a21000 - 0x78a46000 (Other)
[19098105642] [INFO] [kernel::memory] [CPU0]   [57] 0x78a46000 - 0x78a47000 (Reserved)
[19098999051] [INFO] [kernel::memory] [CPU0]   [58] 0x78a47000 - 0x78a53000 (Other)
[19099862067] [INFO] [kernel::memory] [CPU0]   [59] 0x78a53000 - 0x78a54000 (Reserved)
[19100794647] [INFO] [kernel::memory] [CPU0]   [60] 0x78a54000 - 0x78a61000 (Other)
[19101667596] [INFO] [kernel::memory] [CPU0]   [61] 0x78a61000 - 0x78a62000 (Reserved)
[19102558497] [INFO] [kernel::memory] [CPU0]   [62] 0x78a62000 - 0x78aaa000 (Other)
[19103416002] [INFO] [kernel::memory] [CPU0]   [63] 0x78aaa000 - 0x78aab000 (Reserved)
[19104687162] [INFO] [kernel::memory] [CPU0] HHDM Offset: 0xffff800000000000
[19473061179] [CONTRACT] [kernel::memory] [CPU0] Frame allocator initialized with 485781 free frames
[19488051891] [INFO] [bran::arch] [CPU0] IOAPIC: hhdm=0xffff800000000000
[19497306279] [INFO] [bran::arch] [CPU0] IOAPIC: Disabling legacy PIC...
[19499035611] [INFO] [bran::arch] [CPU0] IOAPIC: PIC disabled OK
[19500287928] [INFO] [bran::arch] [CPU0] IOAPIC: RSDP virt=0x7f77e014
[19505186844] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: total length 144
[19507420020] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77802c
[19509114537] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778034
[19510279272] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77803c
[19511414802] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778044
[19512552510] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 1, len 12 at 0xffff80007f77804c
[19514134101] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778058
[19515600258] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778062
[19516759251] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f77806c
[19517897685] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778076
[19519024767] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778080
[19520183595] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 4, len 6 at 0xffff80007f77808a
[19521866628] [INFO] [bran::arch] [CPU0] IOAPIC: MADT parsed OK
[19524047961] [INFO] [bran::arch] [CPU0] SMP: Found 4 CPUs (CPU_COUNT now = 4)
[19526086965] [INFO] [bran::arch] [CPU0] IOAPIC: Found at phys 0xfec00000, GSI base 0
[19528804020] [INFO] [bran::arch] [CPU0] IOAPIC: Registers initialized
[19530417555] [INFO] [bran::arch] [CPU0] IOAPIC: Local APIC enabled (SVR=0x1FF, TPR=0)
[19536751773] [INFO] [bran::arch] [CPU0] IOAPIC: version 0x20, 24 redir entries
[19540054941] [INFO] [bran::arch] [CPU0] IOAPIC: All pins masked
[19546337943] [INFO] [bran::arch] [CPU0] IOAPIC: IRQ1 -> GSI 1 -> 0x21
[19551195312] [INFO] [bran::arch] [CPU0] IOAPIC: IRQ12 -> GSI 12 -> 0x2C
[19552109907] [INFO] [bran::arch] [CPU0] IOAPIC: Init complete
[19553459376] [CONTRACT] [kernel] [CPU0] Initializing global allocator...
[20167762197] [INFO] [kernel::memory::global_alloc] [CPU0] Global allocator initialized (LinkedHeap, 32MB)
[20175108558] [CONTRACT] [kernel] [CPU0] Seeding entropy pool...
[20181231147] [INFO] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (NOT seeded)
[20187281268] [INFO] [kernel::entropy] [CPU0] ENTROPY: marked seeded with weak entropy (timer-only fallback)
[20188642749] [CONTRACT] [kernel] [CPU0] Initializing SIMD...
[20191301031] [CONTRACT] [kernel] [CPU0] Initializing tasking...
[20221728153] [INFO] [kernel::sched] [CPU0]   Acquiring scheduler lock...
[20226666768] [INFO] [kernel::sched] [CPU0]   Lock acquired, checking if initialized...
[20227968915] [INFO] [kernel::sched] [CPU0]   Allocating scheduler...
[20231084016] [INFO] [kernel::sched] [CPU0]   Leaking scheduler...
[20232032799] [INFO] [kernel::sched] [CPU0]   Initializing boot task...
[20236286862] [INFO] [kernel::sched] [CPU0]   Creating boot task...
[20249808249] [INFO] [kernel::sched] [CPU0]   Creating idle tasks...
[20253276549] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 1 assigned to CPU 0
[20277636819] [INFO] [kernel::sched] [CPU0]   Boot task initialized
[20282448615] [INFO] [kernel::sched] [CPU0]   Storing scheduler pointer...
[20307477729] [INFO] [kernel::sched] [CPU0]   Initialized 4 event ring(s)
[20315602824] [CONTRACT] [kernel::sched] [CPU0] Scheduler initialized
[20322086235] [CONTRACT] [alloc] [CPU0] MEDIUM ALLOC #1: 224 KB align=8 total=0MB
[20324293572] [INFO] [kernel::task] [CPU0]   Creating graph worker tasks...
[20326102071] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 2 assigned to CPU 0
[20331430779] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 3 assigned to CPU 0
[20341240623] [INFO] [kernel] [CPU0] System initialized. Setting up preemption timer (100Hz)...
[20381220783] [INFO] [bran::arch::x86_64] [CPU0] LAPIC: calibrated timer (62227400 ticks/sec), init_cnt=622274 for 100Hz
[20391657726] [INFO] [kernel] [CPU0] Kernel: Detected 4 CPUs. Starting 3 secondaries...
[20393020197] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting 3 secondary CPUs...
[20394948222] [INFO] [bran::arch::x86_64] [CPU0] SMP: Initializing toline at 0x8000
[20403903366] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 1 (APIC 1)
[20440414335] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 1 (APIC 1) is online
[20442046944] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 2 (APIC 2)
[20445909198] [INFO] [kernel] [CPU1] SMP: Entering kernel_secondary_entry for CPU 1
[20447992521] [INFO] [kernel::sched] [CPU1] SMP: CPU 1 online (triggered by scheduler spawn)
[20449495308] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 4 assigned to CPU 1
[20453078580] [INFO] [kernel::sched] [CPU1] SMP: Secondary CPU 1 online!
[20454283311] [INFO] [kernel::task] [CPU1] SMP: CPU 1 bootstrapped with idle task 4
[20481497652] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 2 (APIC 2) is online
[20482820886] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 3 (APIC 3)
[20484105543] [INFO] [kernel] [CPU2] SMP: Entering kernel_secondary_entry for CPU 2
[20485690698] [INFO] [kernel::sched] [CPU2] SMP: CPU 2 online (triggered by scheduler spawn)
[20487654957] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 5 assigned to CPU 2
[20489724750] [INFO] [kernel::sched] [CPU2] SMP: Secondary CPU 2 online!
[20490558858] [INFO] [kernel::task] [CPU2] SMP: CPU 2 bootstrapped with idle task 5
[20518477749] [INFO] [kernel] [CPU3] SMP: Entering kernel_secondary_entry for CPU 3
[20519800224] [INFO] [kernel::sched] [CPU3] SMP: CPU 3 online (triggered by scheduler spawn)
[20524723164] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 3 (APIC 3) is online
[20525976273] [INFO] [bran::arch::x86_64] [CPU0] SMP: Secondary CPU startup complete
[20526971157] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 6 assigned to CPU 3
[20528299704] [INFO] [kernel] [CPU0] Kernel: Secondary CPU bring-up complete.
[20529571227] [INFO] [kernel::sched] [CPU3] SMP: Secondary CPU 3 online!
[20530605612] [INFO] [kernel::task] [CPU3] SMP: CPU 3 bootstrapped with idle task 6
[20541700938] [INFO] [kernel::root] [CPU0] Spawning Root service...
[20543296884] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 7 assigned to CPU 1
[20547729444] [INFO] [kernel::root::boot_register] [CPU0] ROOT: boot registration begin (Census Phase 1 v0.2)
[20554233414] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 1. Host creation start
[20557734813] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
RM
[20560164933] [INFO] [kernel::root::service] [CPU1] ROOT: started once
[20561132823] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Initializing components...
[20563548819] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Graph initialized
[20565630228] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Journal initialized
[20566842318] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Interner initialized
[20586910311] [CONTRACT] [kernel::root::service] [CPU1] ROOT: LogSymbols initialized
[20591323632] [CONTRACT] [kernel::root::service] [CPU1] ROOT: BatchScratch initialized
[20593356663] [CONTRACT] [kernel::root::service] [CPU1] ROOT: QueryScratch initialized
[20594165922] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Entering main loop
[20624873742] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[20644284441] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[20647602426] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[20651598363] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[20653415772] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[20655948588] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[20658769362] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[20661275052] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 1. Host registered: t1
[20663502024] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 2. Platform Bus start
[20675547057] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[20679943614] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[20687197674] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[20693004618] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[20705628306] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 2. Platform Bus done
[20710541115] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 3. Kernel start
[20738878875] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 3. Kernel done
[20745527055] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 4. Root Service start
[20759602875] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 4. Root Service done
[20762064147] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 5. CPUs start
[20814495603] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 5. CPUs done
[20825048673] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 6. Memory Ranges start
[21503203128] [CONTRACT] [alloc] [CPU1] MEDIUM ALLOC #2: 120 KB align=8 total=1MB
[22416408168] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 6. Memory Ranges done
[22495375287] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=1000 nodes=331 watches=0 history=498 journal=423 symbols=51 drops=0
[22562245827] [CONTRACT] [alloc] [CPU1] MEDIUM ALLOC #3: 240 KB align=8 total=1MB
[24855502947] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=2000 nodes=446 watches=0 history=960 journal=768 symbols=94 drops=0
[26241307059] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: filesystem bootstrap: /initrd/ with 39 files
[26416104759] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: Census Phase 2: PCI
[26417683116] [INFO] [kernel::root::pci] [CPU0] PCI: Starting enumeration...
[26619844251] [INFO] [kernel::root::pci] [CPU0] PCI: 00:00.0 8086:29c0 Intel Corporation 82G33/G31/P35/P31 Express DRAM Controller class=06:00 prog_if=00 rev=00
[26757019377] [INFO] [kernel::root::pci] [CPU0] PCI: 00:01.0 1af4:1050 Red Hat, Inc. Virtio 1.0 GPU class=03:00 prog_if=00 rev=01
[26829405735] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR0: phys=0x80000000 size=0x800000
[26836469022] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR2: phys=0xc000000000 size=0x4000
[26837489052] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR4: phys=0x80882000 size=0x1000
[26846985297] [INFO] [kernel::root::pci] [CPU0] PCI: Found virtio display controller at 00:01.0
[26876418756] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO notify_cfg BAR2 offset=0x3000 mult=4
[26917589259] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO common_cfg BAR2 offset=0x1000
[26921269221] [INFO] [kernel::root::pci] [CPU0] PCI: Registered virtio GPU (graph_id=1086, idx=1) BAR0=0x80000000
[27010885308] [INFO] [kernel::root::pci] [CPU0] PCI: 00:02.0 1af4:1000 Red Hat, Inc. Virtio network device class=02:00 prog_if=00 rev=00
[27039531849] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR1: phys=0x80881000 size=0x1000
[27041458884] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR4: phys=0xc000004000 size=0x4000
[27048141219] [INFO] [kernel::root::pci] [CPU0] PCI: Found virtio network controller at 00:02.0
[27096456420] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO notify_cfg BAR4 offset=0x3000 mult=4
[27127173018] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO common_cfg BAR4 offset=0x0
[27135276333] [INFO] [kernel::root::pci] [CPU0] PCI: Registered virtio network (graph_id=1120, idx=2) BAR4=0xc000004000
[27144136734] [INFO] [kernel::root::pci] [CPU0] VirtIO-Net: Device registered for userspace driver
[27306202044] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.0 8086:2918 Intel Corporation 82801IB (ICH9) LPC Interface Controller class=06:01 prog_if=00 rev=02
[27316360995] [INFO] [kernel::root::pci] [CPU0] PCI: Found LPC/ISA bridge at 00:1f.0
[27525761307] [INFO] [kernel::root::pci] [CPU0] LPC: Created Legacy IO bus with CMOS and PS/2 controller
[27680479299] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.2 8086:2922 Intel Corporation 82801IR/IO/IH (ICH9R/DO/DH) 6 port SATA Controller [AHCI mode] class=01:06 prog_if=01 rev=02
[27699056385] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR5: phys=0x80880000 size=0x1000
[27705758949] [INFO] [kernel::root::pci] [CPU0] PCI: Found AHCI SATA controller at 00:1f.2
[27709300311] [INFO] [kernel::root::pci] [CPU0] PCI: Registered AHCI controller (graph_id=1193, idx=5) BAR5=0x80880000
[27737301207] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=3000 nodes=584 watches=0 history=1024 journal=1024 symbols=181 drops=0
[27841475343] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.3 8086:2930 Intel Corporation 82801I (ICH9 Family) SMBus Controller class=0c:05 prog_if=00 rev=02
[27849493254] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: registered items. host=t1 kernel=tb
[27851004786] [CONTRACT] [kernel] [CPU0] KERNEL: root census complete: host=t1 kernel=tb root=te
[27852409002] [CONTRACT] [kernel] [CPU0] Kernel: Enumerating 39 boot modules...
[27853892352] [CONTRACT] [kernel] [CPU0]   [0] name='/boot/sprout' cmdline='init' size=111120
[27855282015] [CONTRACT] [kernel] [CPU0]   [1] name='/boot/bristle' cmdline='init' size=41456
[27856351974] [CONTRACT] [kernel] [CPU0]   [2] name='/boot/rtc_cmos' cmdline='init' size=29248
[27857374578] [CONTRACT] [kernel] [CPU0]   [3] name='/boot/clock' cmdline='' size=61968
[27858391440] [CONTRACT] [kernel] [CPU0]   [4] name='/boot/taskman' cmdline='' size=70160
[27859321479] [CONTRACT] [kernel] [CPU0]   [5] name='/boot/ps2_kbd' cmdline='init' size=25072
[27860348868] [CONTRACT] [kernel] [CPU0]   [6] name='/boot/echo' cmdline='init' size=25072
[27862053153] [CONTRACT] [kernel] [CPU0]   [7] name='/boot/bloom' cmdline='init' size=965128
[27863090871] [CONTRACT] [kernel] [CPU0]   [8] name='/boot/ps2_mouse' cmdline='init' size=29168
[27864101034] [CONTRACT] [kernel] [CPU0]   [9] name='/boot/display_bootfb' cmdline='init' size=29248
[27865816572] [CONTRACT] [kernel] [CPU0]   [10] name='/boot/display_virtio_gpu' cmdline='init' size=53864
[27867006387] [CONTRACT] [kernel] [CPU0]   [11] name='/boot/fontd' cmdline='init' size=188944
[27868069251] [CONTRACT] [kernel] [CPU0]   [12] name='/boot/blossom' cmdline='init' size=123408
[27869147724] [CONTRACT] [kernel] [CPU0]   [13] name='/boot/flytrap' cmdline='' size=291352
[27870196992] [CONTRACT] [kernel] [CPU0]   [14] name='/boot/virtio_netd' cmdline='' size=49680
[27871246194] [CONTRACT] [kernel] [CPU0]   [15] name='/boot/rtl8168d' cmdline='' size=45584
[27872258106] [CONTRACT] [kernel] [CPU0]   [16] name='/boot/netd' cmdline='' size=147984
[27873274440] [CONTRACT] [kernel] [CPU0]   [17] name='/boot/fetchd' cmdline='' size=41488
[27874304535] [CONTRACT] [kernel] [CPU0]   [18] name='/boot/anther' cmdline='' size=586272
[27875311860] [CONTRACT] [kernel] [CPU0]   [19] name='/boot/fortune' cmdline='' size=61968
[27876299517] [CONTRACT] [kernel] [CPU0]   [20] name='/boot/photosynthesis' cmdline='' size=12648
[27877335519] [CONTRACT] [kernel] [CPU0]   [21] name='/boot/ahci_disk' cmdline='' size=45664
[27878310801] [CONTRACT] [kernel] [CPU0]   [22] name='/boot/iso9660d' cmdline='' size=53856
[27879305718] [CONTRACT] [kernel] [CPU0]   [23] name='/boot/virtio_sound' cmdline='init' size=49648
[27880437321] [CONTRACT] [kernel] [CPU0]   [24] name='/boot/hdaudio' cmdline='' size=37360
[27881415639] [CONTRACT] [kernel] [CPU0]   [25] name='/boot/pci_stubd' cmdline='' size=29168
[27882469230] [CONTRACT] [kernel] [CPU0]   [26] name='/boot/beeper' cmdline='init' size=33296
[27883501965] [CONTRACT] [kernel] [CPU0]   [27] name='/boot/nectar' cmdline='init' size=176656
[27884571363] [CONTRACT] [kernel] [CPU0]   [28] name='/boot/hello_std' cmdline='' size=16880
[27885620994] [CONTRACT] [kernel] [CPU0]   [29] name='/boot/stdio_demo' cmdline='' size=16880
[27886714185] [CONTRACT] [kernel] [CPU0]   [30] name='/boot/telnetd' cmdline='' size=57872
[27887896707] [CONTRACT] [kernel] [CPU0]   [31] name='/assets/wallpapers/clouds.bmp' cmdline='' size=786486
[27889197765] [CONTRACT] [kernel] [CPU0]   [32] name='/assets/wallpapers/flower.bmp' cmdline='' size=8386614
[27890454339] [CONTRACT] [kernel] [CPU0]   [33] name='/assets/wallpapers/leather.bmp' cmdline='' size=1179702
[27891811728] [CONTRACT] [kernel] [CPU0]   [34] name='/assets/wallpapers/linen.bmp' cmdline='' size=4718646
[27892969434] [CONTRACT] [kernel] [CPU0]   [35] name='/assets/fonts/NotoSans-Regular.ttf' cmdline='' size=569208
[27894632568] [CONTRACT] [kernel] [CPU0]   [36] name='/assets/themes/genie_circles.wasm' cmdline='' size=2824
[27895806708] [CONTRACT] [kernel] [CPU0]   [37] name='/assets/cursors/future/default.svg' cmdline='' size=3051
[27896970948] [CONTRACT] [kernel] [CPU0]   [38] name='/boot/locale.conf' cmdline='' size=85
[27899861979] [INFO] [kernel] [CPU0] Found init module: /boot/sprout (cmdline: 'init'), loading...
[27902703642] [INFO] [kernel::task::loader] [CPU0] Loading module: /boot/sprout
[27903999123] [DEBUG] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[27917734911] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[27941539461] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=213000 exec=false
[27948423294] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=21a000 exec=false
[27964420440] [INFO] [kernel] [CPU0] Spawning sprout with registry at 0x600000...
[27965594580] [CONTRACT] [kernel] [CPU0] Spawning init process...
[27969120861] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 8 (user task/process) assigned to CPU 2
[27978232161] [CONTRACT] [kernel] [CPU0] Entering scheduler loop.
[27979063365] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600000
ThingOS Petals
type 'help' for commands

petals> [27988579806] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=8 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369013504 RFLAGS_BEFORE=134 CR3_BEFORE=50393088 fs_base=0 gs_base=18446744071564586608
[27999677739] [DEBUG] [kernel::sched::sleep] [CPU0] DIAG ctx_switch: cpu=0 from=0 to=2
[28032260256] [INFO] [sprout] [CPU2] [sprout] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ff990 rip=0x20139f rflags=0x206
[28044565428] [INFO] [sprout] [CPU2] SPROUT: v0.4 starting (Supervisor Mode)...
[28046384916] [INFO] [sprout::devtree] [CPU2] SPROUT: devtree::init entry (v0.2)
[28048031682] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 1: Find Host
[28076535168] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 2: HHDM
[28095302499] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 3: Platform Bus
[28101266655] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 4: Firmware
[28103403273] [INFO] [sprout::devtree] [CPU2] SPROUT: Finding ACPI...
[28119805824] [INFO] [sprout::devtree] [CPU2] SPROUT: Found 1 ACPI nodes
[28125123378] [INFO] [sprout::devtree] [CPU2] SPROUT: ACPI RSDP = 0x7f77e014
[28128714834] [INFO] [sprout::devtree] [CPU2] SPROUT: Finding DTB...
[28163298966] [INFO] [sprout::devtree] [CPU2] SPROUT: Found 0 DTB nodes
[28165148484] [INFO] [sprout::devtree] [CPU2] SPROUT: Init OK, returning context
[28166983449] [INFO] [sprout::devtree] [CPU2] SPROUT: build() called
[28168726278] [INFO] [sprout::devtree::x86_64] [CPU2] SPROUT: x86_64 platform enrichment... (v0.2)
[28205066340] [INFO] [sprout::devtree::x86_64] [CPU2] SPROUT: x86_64 enumerate done
[28216715538] [INFO] [sprout] [CPU2] SPROUT: About to create Supervisor...
[28218340392] [INFO] [sprout] [CPU2] SPROUT: Supervisor created, calling run_forever...
[28220333031] [INFO] [sprout::supring modules...
[28266265269] [INFO] [sprout::supervisor] [CPU2] SPROUT: Found 39 modules
[28342820979] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[0] = '/boot/sprout'
[28378799889] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[1] = '/boot/bristle'
[28414349733] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[2] = '/boot/rtc_cmos'
[28454421468] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[3] = '/boot/clock'
[28459277649] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/clock
[28477664985] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[4] = '/boot/taskman'
[28507753461] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[5] = '/boot/ps2_kbd'
[28516387086] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[6] = '/boot/echo'
[28533327933] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[7] = '/boot/bloom'
[28539605589] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[8] = '/boot/ps2_mouse'
[28546501566] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[9] = '/boot/display_bootfb'
[28553464038] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[10] = '/boot/display_virtio_gpu'
[28560986619] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[11] = '/boot/fontd'
[28568066208] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[12] = '/boot/blossom'
[28576301919] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[13] = '/boot/flytrap'
[28587015534] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[14] = '/boot/virtio_netd'
[28594289328] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[15] = '/boot/rtl8168d'
[28601004069] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[16] = '/boot/netd'
[28607153388] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[17] = '/boot/fetchd'
[28613375868] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[18] = '/boot/anther'
[28619712462] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[19] = '/boot/fortune'
[28625758689] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[20] = '/boot/photosynthesis'
[28632214050] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[21] = '/boot/ahci_disk'
[28638376998] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[22] = '/boot/iso9660d'
[28644608256] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[23] = '/boot/virtio_sound'
[28651166313] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[24] = '/boot/hdaudio'
[28657784133] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[25] = '/boot/pci_stubd'
[28663981170] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[26] = '/boot/beeper'
[28670175699] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[27] = '/boot/nectar'
[28676435865] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[28] = '/boot/hello_std'
[28687498917] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/hello_std
[28692879072] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[29] = '/boot/stdio_demo'
[28712994486] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[30] = '/boot/telnetd'
[28718153112] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[31] = '/assets/wallpapers/clouds.bmp'
[28723487133] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[32] = '/assets/wallpapers/flower.bmp'
[28729543821] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[33] = '/assets/wallpapers/leather.bmp'
[28736277669] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[34] = '/assets/wallpapers/linen.bmp'
[28743766095] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[35] = '/assets/fonts/NotoSans-Regular.ttf'
[28767421617] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[36] = '/assets/themes/genie_circles.wasm'
[28786338207] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[37] = '/assets/cursors/future/default.svg'
[28800382974] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[38] = '/boot/locale.conf'
[28811601225] [INFO] [sprout::registry] [CPU2] SPROUT: Scanning boot modules...
[28883805522] [INFO] [sprout::registry] [CPU2] SPROUT: Registering driver 'dev.rtc.Cmos' -> '/boot/rtc_cmos' (fallback)
[29174948187] [INFO] [sprout::registry] [CPU2] SPROUT: Registry scan complete. Found 1 drivers.
[29187050541] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/pci_stubd
[29191689285] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[29196691227] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[29205027093] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[29213210433] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[29231189130] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 9 (user task/process) assigned to CPU 3
[29249432025] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db400
[29250966789] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned pci_stubd (PID=9)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[29253126111] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=9 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369079040 RFLAGS_BEFORE=134 CR3_BEFORE=50860032 fs_base=0 gs_base=18446744071564586640
[29258870751] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/rtc_cmos
[29260761288] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[29262879954] [INFO] [pci_stubd] [CPU3] pci_stubd: starting pci-id matcher
[29264255625] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[29270710062] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=204000 exec=false
[29273281950] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[29293145607] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 10 (user task/process) assigned to CPU 1
[29297326443] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db400
[29301362673] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned rtc_cmos (PID=10)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x49b
[29303337459] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=10 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369144576 RFLAGS_BEFORE=134 CR3_BEFORE=50974720 fs_base=0 gs_base=18446744071564586576
[29309125164] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up storage pipeline...
[29312958708] [INFO] [rtc_cmos] [CPU1] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ffeb0 rip=0x201027 rflags=0x202
[29315510334] [INFO] [rtc_cmos] [CPU1] Starting... arg=49b
[29319683118] [INFO] [rtc_cmos] [CPU1] Serving device ID: ThingId([155, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[29327227017] [INFO] [rtc_cmos] [CPU1] RTC: 2026-04-06 01:56:41 = 1775440601 unix_secs
[29329694229] [INFO] [kernel::time] [CPU1] System clock anchored: unix_secs=1775440601, mono_ns=14664473010, offset=1775440586335526990ns
[29332981458] [INFO] [rtc_cmos] [CPU1] System clock anchored
[29370548724] [INFO] [rtc_cmos] [CPU1] RTC: Set sys.TimeState = 1 (Anchored)
[29523546327] [INFO] [rtc_cmos] [CPU1] Publishing time. Entering maintenance loop.
[29552157921] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ahci_disk
[29553166830] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[29556759573] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[29570493711] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[29572860438] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20a000 exec=false
[29587044498] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 11 (user task/process) assigned to CPU 2
[29609598810] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ahci_disk (PID=11)
[29631559485] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[29634181665] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=11 target_pc=2097152 target_sp=8388608 target_cs=69210720 RFLAGS_BEFORE=134 CR3_BEFORE=51089408 fs_base=0 gs_base=18446744071564586608
[29656131648] [INFO] [ahci_disk] [CPU2] AHCI: Starting AHCI/SATA disk driver v1
[29695032081] [INFO] [sprout::pipelines] [CPU2] SPROUT: Failed to spawn ata_disk: ENOENT
[29706812949] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/iso9660d
[29711215611] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[29713589862] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[29726625027] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[29737388142] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[29752264575] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 12 (user task/process) assigned to CPU 3
[29771401737] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned iso9660d (PID=12)
[29782170000] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up audio driver...
[29783440038] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[29786417166] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=12 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369277472 RFLAGS_BEFORE=130 CR3_BEFORE=51228672 fs_base=0 gs_base=18446744071564586640
[29799636042] [INFO] [iso9660d] [CPU3] ISO9660D: Starting ISO9660 mount service...
[29802442725] [INFO] [ahci_disk] [CPU2] AHCI: Found 6 PCI functions
[29860330929] [INFO] [sprout::pipelines] [CPU2] SPROUT: No Sound device found
[29863029867] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up display pipeline...
[29864648484] [INFO] [iso9660d] [CPU3] ISO9660D: Found 0 block devices
[29866604460] [INFO] [iso9660d] [CPU3] ISO9660D: No ISO9660 filesystems found
[29885533953] [INFO] [iso9660d] [CPU3] ISO9660D: Entering service loop with 0 mounts
[29924172564] [INFO] [sprout::pipelines] [CPU2] SPROUT: Using VirtIO GPU at 1920x1080
[29933384613] [INFO] [sprout::pipelines] [CPU2] SPROUT: Display backend: VirtIO-GPU (1920x1080 stride=7680)
[30722787480] [INFO] [ahci_disk] [CPU2] AHCI: Found AHCI controller at PCI func ThingId([169, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[30728632506] [INFO] [ahci_disk] [CPU2] AHCI: BAR5=0x80880000
[30740247615] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 11 claimed device 1193 (handle 0)
[30745395879] [INFO] [ahci_disk] [CPU2] AHCI: Claimed PCI device ThingId([169, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) handle=0
[30755353068] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR5 phys=0x80880000 size=0x1000 -> virt=0x10000000
[30762570795] [INFO] [ahci_disk] [CPU2] AHCI: Mapped ABAR at 0x10000000
[30765990123] [INFO] [ahci_disk] [CPU2] AHCI: Version 1.0, 6 ports, 32 slots, 64-bit: true
[30769174095] [INFO] [ahci_disk] [CPU2] AHCI: Ports implemented: 0x3f
[30778333476] [INFO] [ahci_disk] [CPU2] AHCI: DMA virt=0x20b000
[30810210321] [INFO] [ahci_disk] [CPU2] AHCI: DMA phys=0x30c8000
[30820146126] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 0...
[30823155429] [INFO] [ahci_disk] [CPU2] AHCI: Port 0 - no device
[30824694252] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 1...
[30826121700] [INFO] [ahci_disk] [CPU2] AHCI: Port 1 - no device
[30828060516] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 2...
[30830277951] [INFO] [ahci_disk] [CPU2] AHCI: Port 2 - SATAPI drive (sig=0xeb140101)
[30843929919] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/display_virtio_gpu
[30844979847] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[30848204112] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[30859419525] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[30863163078] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[30873695556] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 13 (user task/process) assigned to CPU 1
[30901775586] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x30002
[30904869501] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned display driver '/display_virtio_gpu' (PID=13)
[30909144618] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=13 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369352656 RFLAGS_BEFORE=130 CR3_BEFORE=59666432 fs_base=0 gs_base=18446744071564586576
[30930919668] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: starting (drv_req_r=2, drv_resp_w=3)
[30953226777] [INFO] [kernel::syscall::handlers::device] [CPU1] DEVICE: task 13 claimed device 1108 (handle 1)
[30969169242] [INFO] [ahci_disk] [CPU2] AHCI: Registered ATAPI block device 1235 port=2 model='                                        ' rpc_port=5
[31009307142] [INFO] [virtio_gpu] [CPU1] virtio_gpu: caps from graph - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[31011929190] [INFO] [kernel::syscall::handlers::device] [CPU1] DEVICE: Mapped BAR2 phys=0xc000000000 size=0x4000 -> virt=0x10001000
[31025581818] [INFO] [virtio_gpu] [CPU1] virtio_gpu: device features=0x30000002 virgl=false
[31039900650] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: GPU initialized successfully
[31048572720] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: Virgl 3D not supported, using 2D only
[31063290753] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up input pipeline (keyboard + mouse)...
[31067461194] [INFO] [sprout::pipelines] [CPU2] SPROUT: Created kbd_raw port (w=7, r=8)
[31071605994] [INFO] [sprout::pipelines] [CPU2] SPROUT: Created mouse_raw port (w=9, r=10)
[31073928402] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: creating frame pool 1x 1920x1080 stride=7680 format=2
[31076332947] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ps2_kbd
[31077500619] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[31078965951] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[31098231450] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=203000 exec=false
[31101517491] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[31115693334] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 14 (user task/process) assigned to CPU 2
[31124517765] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 3...
[31130541222] [INFO] [ahci_disk] [CPU2] AHCI: Port 3 - no device
[31132696452] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 4...
[31133973552] [INFO] [ahci_disk] [CPU2] AHCI: Port 4 - no device
[31135232172] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 5...
[31136467857] [INFO] [ahci_disk] [CPU2] AHCI: Port 5 - no device
[31141808379] [INFO] [ahci_disk] [CPU2] AHCI: Found 1 SATA disk(s)
[31168040409] [INFO] [ahci_disk] [CPU2] AHCI: Entering RPC service loop
[31187040885] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00fff90
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x7
[31188937164] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=14 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369434176 RFLAGS_BEFORE=130 CR3_BEFORE=60096512 fs_base=0 gs_base=18446744071564586608
[31203642690] [INFO] [ps2_kbd] [CPU2] ps2_kbd: online (handle=7)
[31234491882] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ps2_kbd (PID=14)
[31237462014] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ps2_mouse
[31238495178] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[31241170488] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[31249308981] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[31251264693] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[31263197130] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 15 (user task/process) assigned to CPU 3
[31268299425] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ps2_mouse (PID=15)
[31275049344] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00fff90
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x9
[31277431482] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=15 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369500256 RFLAGS_BEFORE=134 CR3_BEFORE=68501504 fs_base=0 gs_base=18446744071564586640
[31296172215] [INFO] [ps2_mouse] [CPU3] ps2_mouse: online (handle=9)
[31297925934] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/bristle
[31299134130] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[31301633517] [INFO] [ps2_mouse] [CPU3] ps2_mouse: enabling aux port
[31304320410] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[31315592616] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[31321014813] [INFO] [ps2_mouse] [CPU3] ps2_mouse: controller cfg already correct (0x47)
[31323274785] [INFO] [ps2_mouse] [CPU3] ps2_mouse: sending RESET (0xFF)
[31325524890] [INFO] [ps2_mouse] [CPU3] ps2_mouse: reset ACK received (0xfa)
[31328489313] [INFO] [ps2_mouse] [CPU3] ps2_mouse: BAT passed (0xaa), ID 0x00 confirmed
[31330642233] [INFO] [ps2_mouse] [CPU3] ps2_mouse: setting sample rate (100)
[31338798150] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[31351833150] [INFO] [ps2_mouse] [CPU3] ps2_mouse: setting resolution (3)
[31352963994] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 16 (user task/process) assigned to CPU 1
[31357278909] [INFO] [ps2_mouse] [CPU3] ps2_mouse: status result = Some(0) Some(3) Some(100)
[31374035352] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned bristle (PID=16)
[31377163785] [INFO] [ps2_mouse] [CPU3] ps2_mouse: sending enable command (0xF4)
[31378323273] [INFO] [sprout::pipelines] [CPU2] SPROUT: Input broker ready (keyboard + mouse)
[31379701815] [INFO] [ps2_mouse] [CPU3] ps2_mouse: enable ACK received (0xfa)
[31387413783] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up network stack...
[31791546336] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00fff90
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x8000a000b000d
[31794239565] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=16 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369583040 RFLAGS_BEFORE=134 CR3_BEFORE=68616192 fs_base=0 gs_base=18446744071564586576
[31809370626] [INFO] [ps2_kbd] [CPU2] ps2_kbd: created driver node 1240
[31815809916] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task subscribed to vector 0x21
[31818750612] [INFO] [ps2_kbd] [CPU2] ps2_kbd: subscribed to IRQ1 (vector 0x21)
[31821823671] [INFO] [bristle] [CPU1] BRISTLE_MAIN_ENTERED_WITH_LOGS_YAY
[31826286195] [INFO] [bristle] [CPU1] bristle: online (kbd=8, mouse=10, evt=11, echo=13)
[31831202040] [INFO] [bristle] [CPU1] bristle: created broadcast topic 0
[31850419293] [INFO] [sprout::pipelines] [CPU2] SPROUT: Found NIC device ThingId([117, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[31856230131] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/virtio_netd
[31857813471] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[31860739053] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[31873362741] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=208000 exec=false
[31877325282] [INFO] [bristle] [CPU1] bristle: registered in graph as svc.Input (id=1241)
[31878806652] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20b000 exec=false
[31891441032] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 17 (user task/process) assigned to CPU 2
[31899246786] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned virtio_netd (PID=17)
[31908265125] [INFO] [sprout::pipelines] [CPU2] SPROUT: spawn_netd start
[31910336568] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/netd
[31911374517] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[31913452197] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[31960793997] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=21d000 exec=false
[31971707097] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=223000 exec=false
[32039726136] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: frame pool ready (1 buffer)
[32262031725] [INFO] [ps2_mouse] [CPU3] ps2_mouse: init done
[32263249920] [INFO] [kernel::syscall::handlers::device] [CPU3] DEVICE: task subscribed to vector 0x2c
[32266108446] [INFO] [ps2_mouse] [CPU3] ps2_mouse: subscribed to IRQ12 (vector 0x2c)
[32268306708] [INFO] [ps2_mouse] [CPU3] ps2_mouse: entering interrupt-driven loop
[33231590700] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 18 (user task/process) assigned to CPU 3
[33236274423] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned netd (PID=18)
[33241831128] [INFO] [ps2_kbd] [CPU2] ps2_kbd: entering cooperative polling loop
[33245890326] [INFO] [ps2_kbd] [CPU2] ps2_kbd: using polling mode (10ms interval)
[33248955234] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00fff90
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x475
[33251691561] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010a1b8
[33253072974] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=17 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369653616 RFLAGS_BEFORE=130 CR3_BEFORE=68743168 fs_base=0 gs_base=18446744071564586608
[33263229153] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Starting VirtIO-NET driver service...
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[33266406096] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Searching for NIC device...
[33267895881] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=18 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369719152 RFLAGS_BEFORE=130 CR3_BEFORE=68878336 fs_base=0 gs_base=18446744071564586640
[33279147132] [INFO] [netd] [CPU3] NETD: Starting network stack service...
[33289359576] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Found NIC device t475
[33295498368] [INFO] [virtio::device] [CPU2] VirtIO: device::new(0x475) - claiming...
[33296777547] [INFO] [netd] [CPU3] NETD: Looking for virtio_netd driver service...
[33298068210] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 17 claimed device 1141 (handle 2)
[33300503313] [INFO] [virtio::device] [CPU2] VirtIO: claimed, handle=2
[33315041496] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 2] Starting Network Apps and Services
[33317826135] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up network apps...
[33320653179] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/anther
[33322090296] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[33324918825] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[33429417318] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=268000 exec=false
[33462394119] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=28e000 exec=false
[33474761826] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 19 (user task/process) assigned to CPU 1
[33478062123] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00fff90
[33479575701] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned anther (PID=19)
[33482346810] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/nectar
[33483825936] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[33486966447] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[33494032044] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=19 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369784688 RFLAGS_BEFORE=130 CR3_BEFORE=79892480 fs_base=0 gs_base=18446744071564586576
[33507254682] [INFO] [anther] [CPU1] anther: Starting HTTP server (ThingOS anther v0.1)
[33511348827] [INFO] [anther] [CPU1] anther: Starting server mode on port 80...
[33518050005] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[33520374294] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[33529785465] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=226000 exec=false
[33534717777] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=22a000 exec=false
[33542957184] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 20 (user task/process) assigned to CPU 2
[33546011433] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned nectar (PID=20)
[33548196363] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/fetchd
[33549104952] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[33551248302] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[33559941723] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[33564467508] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[33576540525] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 21 (user task/process) assigned to CPU 3
[33580174023] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010b5a8
[33582438780] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned fetchd (PID=21)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[33585587244] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=21 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369915760 RFLAGS_BEFORE=130 CR3_BEFORE=80826368 fs_base=0 gs_base=18446744071564586640
[33597713325] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/clock
[33604175088] [INFO] [fetchd] [CPU3] FETCHD: Starting IP address display...
[33605259072] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[33607787994] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[33608972529] [INFO] [fetchd] [CPU3] FETCHD: Waiting for UI Root (Compositor)...
[33620380398] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[33626033595] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20e000 exec=false
[33637502877] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 22 (user task/process) assigned to CPU 1
[33642553791] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned clock (PID=22)
[33645564909] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/taskman
[33646726014] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[33649074327] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[33654205794] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb010b5a8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[33656646210] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=22 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369981744 RFLAGS_BEFORE=134 CR3_BEFORE=80953344 fs_base=0 gs_base=18446744071564586576
[33667367019] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[33672774531] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=210000 exec=false
[33684392214] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 23 (user task/process) assigned to CPU 2
[33689075508] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned taskman (PID=23)
[33693909117] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/fontd
[33694995774] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[33697478265] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[33738378795] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=226000 exec=false
[33745646649] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=22d000 exec=false
[33755252718] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 24 (user task/process) assigned to CPU 3
[33760302510] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned fontd (PID=24)
[33761848098] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010df80
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[33764034876] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/blossom
[33765045534] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[33768961182] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[33771765852] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=24 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370117456 RFLAGS_BEFORE=134 CR3_BEFORE=81256448 fs_base=0 gs_base=18446744071564586640
[33779440497] [INFO] [fontd] [CPU3] FONTD: Starting Graph-Native Font Service v2 (Atlas-based IPC)
[33795578850] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=219000 exec=false
[33802793772] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=21d000 exec=false
[33827226972] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 25 (user task/process) assigned to CPU 1
[33833937522] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned blossom (PID=25)
[33849306480] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb010df80
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[33858156321] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=25 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370199376 RFLAGS_BEFORE=134 CR3_BEFORE=81530880 fs_base=0 gs_base=18446744071564586576
[33872994177] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/flytrap
[33881230251] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[33882584604] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[33888654063] [INFO] [blossom] [CPU1] BLOSSOM: Starting SVG Cache Service
[33890980959] [INFO] [blossom] [CPU1] BLOSSOM: Init UI pipeline...
[33912564873] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[33914721126] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[33916880052] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[33927121437] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=225 subj_lo=0
[33931389096] [INFO] [fontd] [CPU3] FONTD: Service node created, req=15, resp=18
[33940793106] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=237000 exec=false
[33953786427] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[33954675876] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[33955526484] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[33957327426] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=229 pred=0 subj_lo=0
[33972435816] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=246000 exec=false
[33985247868] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 26 (user task/process) assigned to CPU 2
[33991051677] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned flytrap (PID=26)
[33996560664] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00fff90
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[33999308772] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=20 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369850224 RFLAGS_BEFORE=130 CR3_BEFORE=80564224 fs_base=0 gs_base=18446744071564586608
[34041238737] [INFO] [nectar] [CPU2] NECTAR: Started.
[34048508406] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb010b5a8
[34049751747] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[34051708548] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[34058191068] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=23 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370051920 RFLAGS_BEFORE=134 CR3_BEFORE=81100800 fs_base=0 gs_base=18446744071564586608
[34073568111] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[34075104756] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=231 pred=0 subj_lo=0
[34077461847] [INFO] [taskman] [CPU2] TASKMAN: starting task manager
[34080415974] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00f10e8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[34085957037] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[34089606969] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[34091852322] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=26 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370267232 RFLAGS_BEFORE=134 CR3_BEFORE=81739776 fs_base=0 gs_base=18446744071564586608
[34097842086] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[34099311378] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=230 subj_lo=0
[34101051930] [INFO] [flytrap] [CPU2] FLYTRAP: Starting unified content provider service...
[34103808420] [INFO] [flytrap] [CPU2] FLYTRAP: Service contract validated - graph-native asset watcher
[34106261013] [INFO] [flytrap] [CPU2] FLYTRAP: Initializing Limine module content source...
[34114201077] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 3] Starting Compositor
[34165695267] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[34172858016] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[34184697756] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[34186439034] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=234 pred=0 subj_lo=0
[34226924985] [INFO] [fontd] [CPU3] FONTD: Opened ASSET watch (handle=1252) for kind 'Asset'
[34229614584] [INFO] [virtio::device] [CPU2] VirtIO: common_bar=4 common_off=0x0 notify_bar=4 notify_off=0x3000 mult=4
[34243820424] [INFO] [fontd] [CPU3] FONTD: Service ready
[34245713601] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[34247191803] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[34370611242] [INFO] [sprout::pipelines] [CPU2] SPROUT: Writing bloom BS: drv_req=1, drv_resp=4, bristle_evt=12
[34404716511] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[34406998989] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[34409319318] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[34412027793] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=237 subj_lo=0
[34415714256] [INFO] [sprout::pipelines] [CPU2] SPROUT: Bloom handles via BS=1248 backend=VirtIO-GPU
[34444233516] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[34446649809] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[34449038052] [INFO] [virtio::device] [CPU2] VirtIO: mapping common BAR4...
[34452600303] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[34455051312] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR4 phys=0xc000004000 size=0x4000 -> virt=0x107f4000
[34458513441] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=240 subj_lo=0
[34464031932] [INFO] [virtio::device] [CPU2] VirtIO: common_cfg at 0x107f4000
[34470035292] [INFO] [virtio::device] [CPU2] VirtIO: notify_cfg at 0x107f7000
[34472665194] [INFO] [virtio::device] [CPU2] VirtIO: device_cfg = Some(276783104)
[34474006611] [INFO] [virtio::device] [CPU2] VirtIO: allocating DMA command buffer...
[34483466622] [INFO] [virtio::device] [CPU2] VirtIO: cmd_buf virt=0x107f8000 phys=0x4e94000
[34487539152] [INFO] [virtio::device] [CPU2] VirtIO: device::new complete
[34511398218] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[34513384059] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Device features 0x30bf8024
[34514694291] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[34517315514] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[34519967229] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=242 subj_lo=0
[34523017914] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: MAC 52:54:00:12:34:56
[34525807734] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Link UP
[34561703022] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[34569810033] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[34571225568] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[34572887151] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=243 subj_lo=0
[34583312049] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/bloom
[34584391842] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[34586641485] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[34683130218] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[34688616303] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[34692062064] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[34693713912] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=244 pred=0 subj_lo=0
[34727547657] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[34734819537] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[34854311184] [INFO] [blossom] [CPU1] BLOSSOM: UI pipeline ready
[34856909505] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=2d4000 exec=false
[34902252627] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=2ea000 exec=false
[34914708939] [INFO] [blossom] [CPU1] BLOSSOM: Service node created, req=19, resp=22
[34922082624] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 27 (user task/process) assigned to CPU 3
[34925033682] [INFO] [blossom] [CPU1] BLOSSOM: Service ready
[34927418625] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned bloom (PID=27)
[34932398061] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/echo
[34933308234] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[34935831513] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db4a8
[34936818873] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x4e0
[34941483192] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=27 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370364352 RFLAGS_BEFORE=134 CR3_BEFORE=82415616 fs_base=0 gs_base=18446744071564586640
[34950572580] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=204000 exec=false
[34952426454] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[34962589563] [INFO] [bloom::logging] [CPU3] bloom: logging initialized
[34971873585] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 28 (user task/process) assigned to CPU 1
[34977293736] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned echo (PID=28)
[34980731148] [INFO] [sprout::pipelines] [CPU2] SPROUT: Compositor ready
[34989904950] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db4a8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xe
[34992415029] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=28 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370433984 RFLAGS_BEFORE=134 CR3_BEFORE=83464192 fs_base=0 gs_base=18446744071564586576
[35001248997] [INFO] [echo] [CPU1] echo: starting up
[35012278785] [INFO] [echo] [CPU1] echo: ready for Bristle events (main loop using handle 14)
[35030720109] [INFO] [bloom] [CPU3] bloom: [bloom] SIMD backend: SSE2 (x86_64)
[35086285707] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX queue 0 setup (size=32)
[35117912940] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX queue 1 setup (size=32)
[35128929033] [INFO] [bloom] [CPU3] [bloom] EARLY boot args: bristle_evt=12 arg_req=1 arg_resp=4
[35131899528] [INFO] [bloom::compositor] [CPU3] bloom: map_from_bytespace start bs=1233
[35135088846] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Allocated RX pool (16 pages, 32 buffers)
[35139998685] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Allocated TX buffer (1 page)
[35143982907] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX queue filled with 32 buffers
[35146873377] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: DRIVER_OK set, device is live
[35149330920] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Driver initialized successfully
[35151712464] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Driver initialized successfully
[35153955111] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: MAC 52:54:00:12:34:56
[35156032989] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Waiting for link...
[35157838815] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Link is UP
[35165928930] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created port (write=23, read=24)
[35173812069] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created RX port (write=25, read=26)
[35200370898] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 4] Proof of life (Beeper)
[35207378349] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawning beeper...
[35209925652] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/beeper
[35211304788] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[35213020821] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[35220613659] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[35223600654] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[35235068088] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 29 (user task/process) assigned to CPU 0
[35239266744] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned beeper (PID=29)
[35243072337] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 5] Starting Discovered User Apps
[35245490379] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/clock'
[35248089822] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/clock
[35249278317] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[35251950987] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[35264006811] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[35269659315] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20e000 exec=false
[35281840176] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 30 (user task/process) assigned to CPU 3
[35286893664] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=30)
[35289189705] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/hello_std'
[35291617416] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/hello_std
[35292550821] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[35294895900] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[35300939685] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=202000 exec=false
[35302694988] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=203000 exec=false
[35304796329] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db4a8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[35307210444] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=30 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370708416 RFLAGS_BEFORE=134 CR3_BEFORE=84320256 fs_base=0 gs_base=18446744071564586640
[35323409187] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[35326159242] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[35331739179] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 31 (user task/process) assigned to CPU 1
[35348460675] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created service node ThingId([235, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[35358894582] [INFO] [bloom::compositor] [CPU3] bloom: symbols loaded
[35361167094] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=31)
[35362369317] [INFO] [bloom::compositor] [CPU3] bloom: loop iter 1
[35364433764] [INFO] [sprout::supervisor] [CPU2] SPROUT: Startup complete. Entering monitor loop.
[35373522855] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb010e620
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[35381857566] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([235, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[35383662468] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=31 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370773952 RFLAGS_BEFORE=134 CR3_BEFORE=84467712 fs_base=0 gs_base=18446744071564586576
[35423319591] [ERROR] [kernel::trap] [CPU1] user_invalid_opcode rip=0x0000000000201420 err=0x0000 rsp=0x00000000007fff08 cs=0x2b kind=6
[35439788307] [INFO] [bloom::compositor] [CPU3] bloom: got width 1920
[35473357656] [INFO] [flytrap] [CPU2] FLYTRAP: Created Limine ContentSource node
[35475639309] [INFO] [flytrap] [CPU2] FLYTRAP: Performing initial boot module scan...
[35545480740] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Published service - TX port=23, RX port=26
[35551980552] [INFO] [bloom::compositor] [CPU3] bloom: got properties w=1920 h=1080 s=7680 f=1
[35568814776] [INFO] [bloom::compositor] [CPU3] bloom: properties OK, building from config...
[35577320163] [INFO] [bloom::compositor] [CPU3] bloom: compositor bytespace 1233 (1920x1080 stride=7680 format=1)
[35626459638] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([235, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[35640918555] [INFO] [bloom::compositor] [CPU3] bloom: display backend: VirtIO-GPU
[35670068346] [INFO] [bloom::compositor] [CPU3] bloom: mapped size=8294400 (source=bytespace_info)
[35771829621] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[35773488960] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[35784945405] [INFO] [bloom::present] [CPU3] bloom: driver REGISTER (kind=2 caps=0x3)
[35789390439] [INFO] [bloom::present] [CPU3] display: negotiated proto 1.0 caps=DIRTY_RECTS|FULLFRAME max_rects=32
[35796881736] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: received MSG_HELLO
[35819039421] [INFO] [netd] [CPU3] NETD: Driver TX port=23, RX port=26, link_up=true, mtu=1500
[35835897372] [INFO] [netd] [CPU3] NETD: Connected to driver - MAC 52:54:00:12:34:56
[35842022172] [INFO] [netd] [CPU3] NETD: Created socket API port (write=27, read=28)
[35877485457] [INFO] [bloom] [CPU3] [bloom] Requesting initial driver buffer...
[35902154145] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/sprout' (raw, 111120 bytes, hash=a121b43754098a0b)
[35928275658] [INFO] [bloom] [CPU3] [bloom] ACQUIRED RETURNED!
[35992726473] [INFO] [bloom] [CPU3] [bloom] ACQUIRED initial driver buffer: 0x11018000 (bs_id=ThingId([214, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]))
[35996306577] [INFO] [bloom] [CPU3] bloom: creating surface...
[35998257405] [INFO] [bloom] [CPU3] bloom: surface created!
[35999499591] [INFO] [bloom] [CPU3] bloom: finding UI CROWN...
[36008266140] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=27, our_write=31, our_read=32)
[36011200269] [INFO] [anther] [CPU1] anther: Connected to network stack
[36020326386] [INFO] [bloom] [CPU3] bloom: creating new UI CROWN
[36075615675] [INFO] [netd] [CPU3] NETD: Published initial stack node ThingId([238, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) to graph
[36106962078] [INFO] [stem::ui] [CPU3] UiBuilder: created root 1263
[36108549675] [INFO] [bloom] [CPU3] bloom: UI CROWN initialized!
[36126074424] [INFO] [taskman] [CPU2] TASKMAN: Found UI Crown (attempt 5)
[36128791908] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 32 (user thread) assigned to CPU 3
[36144211884] [INFO] [bloom] [CPU3] bloom: spawned asset watcher (tid=32)
[36146316921] [INFO] [bloom::frame_loop] [CPU3] bloom: running (fps_target=60)
[36148024671] [INFO] [bloom] [CPU3] [bloom] bristle_evt_handle = 12 (legacy was 12)
T:0270 [36159477354] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] asset_watcher_entry: spawning sub-loaders
[36174029166] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 33 (user thread) assigned to CPU 3
[36189818181] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 34 (user thread) assigned to CPU 3
[36214048794] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 35 (user thread) assigned to CPU 3
[36234528528] [INFO] [netd] [CPU3] NETD: Starting DHCP...
[36238211097] [INFO] [netd::dhcp] [CPU3] DHCP: Starting discovery...
[36249824787] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 304 bytes - IPv4 other
[36262441875] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/bristle' (raw, 41456 bytes, hash=8aec727fb8928c70)
[36308309994] [INFO] [netd::dhcp] [CPU3] DHCP: Deconfigured
T:07D0 T:0640 T:F0B0 [36317908440] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 304 bytes
[36327513882] [INFO] [fetchd] [CPU3] FETCHD: Found UI Root: 1263
[36333520575] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=0 len=600
[36362729601] [INFO] [bloom] [CPU3] bloom: GPU composition not supported (or Virgl disabled) - using CPU composition mode
[36375928281] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 590 byte frame (594 encoded) to netd rx_port=25
[36377922372] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] wallpaper loader: loading flower.bmp
[36391457751] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (594 bytes sent)
[36417562434] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 36 (user thread) assigned to CPU 3
[36428438838] [DEBUG] [bloom::asset] [CPU3] [asset_bank] worker spawned tid=36 (priority=2)
[36432280137] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] cursor loader: loading default cursor
[36436565616] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] icon loader started
[36445967547] [INFO] [bloom] [CPU3] bloom: calling intern for FONT_GLYPH
T:1220 [36456830883] [DEBUG] [bloom::asset] [CPU3] [asset_bank] worker started (priority bump)
[36459548697] [DEBUG] [bloom::asset] [CPU3] [asset_bank] load_wallpaper_immediate: flower.bmp
[36474563697] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 590 bytes
[36497804475] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 316 bytes - IPv4 other
[36504477702] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 316 bytes
[36532612644] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=1 len=600
[36536118927] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 590 byte frame (594 encoded) to netd rx_port=25
[36540467172] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (594 bytes sent)
[36558197082] [INFO] [bloom] [CPU3] bloom: intern returned FONT_GLYPH=285
[36560049009] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for FONT_GLYPH
[36582156204] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[36584063901] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[36586072248] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[36590528238] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=285 subj_lo=0
[36604952538] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 590 bytes
[36620277012] [INFO] [netd::dhcp] [CPU3] DHCP: Configuration received
[36623570610] [INFO] [netd] [CPU3] NETD: DHCP complete - IP: 10.0.2.15, Gateway: 10.0.2.2, DNS: 10.0.2.3
[36655399440] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1278)
[36657820419] [INFO] [bloom] [CPU3] bloom: calling intern for UI_WINDOW
[36688351755] [INFO] [bloom] [CPU3] bloom: intern returned UI_WINDOW=244
[36691117353] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for UI_WINDOW
[36692548002] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[36694166256] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[36695891232] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[36697750848] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=244 pred=0 subj_lo=0
[36706412424] [INFO] [netd] [CPU3] NETD: DHCP configured — IP: 10.0.2.15, GW: 10.0.2.2, DNS: 10.0.2.3
[36711704271] [INFO] [netd] [CPU3] NETD: Network stack ready, entering service loop
[36718788051] [INFO] [netd::socket_api] [CPU3] SOCKET_API: UDP_BIND on port 5353
[36722822598] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Bound UDP on port 5353, handle=1
[36727175661] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_LISTEN on port 80 with backlog 64
[36732647424] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Listening on port 80, handle=2
[36747870027] [INFO] [anther] [CPU1] anther: Listening on port 80 (handle=2)
[36781599789] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1280)
[36783322785] [INFO] [bloom] [CPU3] bloom: calling intern for UI_PAINT_GEN
[36800171595] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Joining multicast group 224.0.0.251
[36810861186] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 42 bytes - IPv4 other
[36831047847] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 42 bytes
[36847594839] [INFO] [bloom] [CPU3] bloom: intern returned UI_PAINT_GEN=293
[36850295229] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for UI_PAINT_GEN
[36852485076] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[36854243415] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[36856355349] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[36858562026] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=293 subj_lo=0
[36872652960] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/rtc_cmos' (raw, 29248 bytes, hash=562b9fa5b03c7a45)
[36911671434] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1283)
[36917165208] [INFO] [bloom] [CPU3] [bloom] Starting UI loop immediately (not waiting for fonts)
[37355064747] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/clock' (raw, 61968 bytes, hash=cae77ff7f85d3f2f)
[37481612256] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[37582237110] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping bytespace 741 (8386614 bytes) for 'flower.bmp'
[37632909501] [DEBUG] [bloom::asset] [CPU3] [asset_bank] decoding BMP for 'flower.bmp'...
[37761908943] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=4000 nodes=664 watches=13 history=1024 journal=1024 symbols=314 drops=0
[37817122134] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/taskman' (raw, 70160 bytes, hash=d3dcbbde76a4f136)
[38060952600] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/ps2_kbd' (raw, 25072 bytes, hash=e5e306bcbc479ba0)
[38294228940] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/echo' (raw, 25072 bytes, hash=015ec546bc28642e)
[38704585359] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/bloom' (raw, 965128 bytes, hash=f2305eb88ad8f119)
[38993019285] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/ps2_mouse' (raw, 29168 bytes, hash=87af446dc7a3dc67)
[39275735829] [DEBUG] [bloom::asset] [CPU3] [asset_bank] BMP decoded: 2048x1365 for 'flower.bmp'
[39318249696] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/display_bootfb' (raw, 29248 bytes, hash=fd9c527f4c1617be)
[39675684411] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/display_virtio_gpu' (raw, 53864 bytes, hash=bc0074690049f9cd)
[40106547162] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/fontd' (raw, 188944 bytes, hash=55b938b69928e656)
[40238670120] [INFO] [taskman] [CPU2] TASKMAN: Window created, found 1 initial tasks
[40248370701] [WARN] [WARN] [CPU2] root_watch_open: WATCH_START_LATEST is deprecated; use start_seq=0 + watch_drain.
[40250679051] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[40252281531] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[40254048153] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=18446744073709551615
[40256147184] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=225 subj_lo=0
[40278897648] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[40280712483] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[40282711953] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=18446744073709551615
[40288302846] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=341 pred=0 subj_lo=0
[40565029560] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/blossom' (raw, 123408 bytes, hash=99095735adb015b8)
[40808521182] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=5000 nodes=725 watches=15 history=1024 journal=1024 symbols=343 drops=0
[41072352024] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/flytrap' (raw, 291352 bytes, hash=bf23e61b1474078c)
[41076687729] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_wallpaper (pending): 2048x1365
[41079107619] [DEBUG] [bloom::asset] [CPU3] [asset_bank] load_cursor_immediate: /assets/cursors/future/default.svg
[41084189355] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[41226118461] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[41306046606] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[41307962850] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[41309756169] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[41311559553] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=307 pred=0 subj_lo=0
[41398618041] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[41404096008] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[41407484547] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[41409625026] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=308 pred=0 subj_lo=0
[41469127029] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[41662420338] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/virtio_netd' (raw, 49680 bytes, hash=e044c79e0fe2b998)
[41668451715] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[41781945909] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[41784370122] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[41790420672] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[41794063839] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=309 pred=0 subj_lo=0
[41829329883] [INFO] [fetchd] [CPU3] FETCHD: Entering main loop, watching for network stack...
[41911847769] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[41916624156] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[41919673983] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[41923951707] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=343 pred=0 subj_lo=0
[41929418949] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[41969789235] [INFO] [fetchd] [CPU3] FETCHD: Got IP address: 10.0.2.15
[42119704374] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[42524492274] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/rtl8168d' (raw, 45584 bytes, hash=a6a9e038341d5a28)
[42560270016] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[42644932077] [INFO] [bloom::present] [CPU3] display: full-frame damage, using full-frame present
[42785760534] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping bytespace 791 (3051 bytes)
[42807461499] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapped to 0x12202000
[42814462284] [DEBUG] [bloom::asset] [CPU3] [asset_bank] detected SVG format
[42824539098] [DEBUG] [bloom::asset] [CPU3] [asset_bank] parsing SVG cursor from /assets/cursors/future/default.svg
[42931904169] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=27 flat_sq=0.003125 edges=36
[42964374255] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=10 flat_sq=0.003125 edges=19
[42971922114] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=60 flat_sq=0.003125 edges=45
[42979162842] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=10 flat_sq=0.003125 edges=32
[42988266717] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=95 flat_sq=0.003125 edges=72
[42995773524] [DEBUG] [bloom::asset] [CPU3] [asset_bank] SUCCESS: SVG cursor rasterized 64x64
[43025425542] [INFO] [bloom] [CPU3] [CONTRACT] [bloom] First frame rendered
[43054575432] [DEBUG] [bloom::reclaimer] [CPU3] [reclaimer] +11182080 bytes (total: 11182080)
[43057845732] [INFO] [bloom::asset] [CPU3] [asset_bank] promoting wallpaper 'flower.bmp' to gen=1 (11182080b)
[43066082961] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_cursor (pending)
[43070726622] [DEBUG] [bloom::asset] [CPU3] [asset_bank] cursor: Static frame 64x64 hotspot (12, 8)
[43072755528] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping font bytespace 771 (569208 bytes)
[43119494946] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapped at 0x12203000
[43122057198] [DEBUG] [bloom::asset] [CPU3] [asset_bank] parsing font '/assets/fonts/NotoSans-Regular.ttf'...
[43284382812] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/netd' (raw, 147984 bytes, hash=0e57f834b3afbcb1)

```
</details>
