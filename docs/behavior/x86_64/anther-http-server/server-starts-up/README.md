# ✅ Scenario: Server starts up

> Last run: 2026-04-05 17:55:01

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | When I start the machine | ✅ | 3848ms | - [📜](./01/serial.log) - |
| 2 | Then I should see a message in the serial output that says "anther: Listening on port 80" | ✅ | 2734ms | - [📜](./02/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[11897208675] [INFO] [kernel] [CPU0] BOOTFB: width=1920 height=1080 pitch=7680 bpp=32 format=Bgrx8888
[11902830522] [CONTRACT] [kernel] [CPU0] thing-os kernel starting...
[11906536026] [INFO] [bran::requests] [CPU0] Limine: Found 39 boot modules
[11908562160] [INFO] [bran::requests] [CPU0]   [0] /boot/sprout (cmdline='init') size=111120
[11909774085] [INFO] [bran::requests] [CPU0]   [1] /boot/bristle (cmdline='init') size=41456
[11910423393] [INFO] [bran::requests] [CPU0]   [2] /boot/rtc_cmos (cmdline='init') size=29248
[11911449759] [INFO] [bran::requests] [CPU0]   [3] /boot/clock (cmdline='') size=61968
[11912256114] [INFO] [bran::requests] [CPU0]   [4] /boot/taskman (cmdline='') size=70160
[11913128964] [INFO] [bran::requests] [CPU0]   [5] /boot/ps2_kbd (cmdline='init') size=25072
[11913967758] [INFO] [bran::requests] [CPU0]   [6] /boot/echo (cmdline='init') size=25072
[11914772265] [INFO] [bran::requests] [CPU0]   [7] /boot/bloom (cmdline='init') size=965128
[11915553771] [INFO] [bran::requests] [CPU0]   [8] /boot/ps2_mouse (cmdline='init') size=29168
[11916347091] [INFO] [bran::requests] [CPU0]   [9] /boot/display_bootfb (cmdline='init') size=29248
[11917044117] [INFO] [bran::requests] [CPU0]   [10] /boot/display_virtio_gpu (cmdline='init') size=53864
[11917811202] [INFO] [bran::requests] [CPU0]   [11] /boot/fontd (cmdline='init') size=188944
[11918445462] [INFO] [bran::requests] [CPU0]   [12] /boot/blossom (cmdline='init') size=123408
[11919089523] [INFO] [bran::requests] [CPU0]   [13] /boot/flytrap (cmdline='') size=291352
[11919748995] [INFO] [bran::requests] [CPU0]   [14] /boot/virtio_netd (cmdline='') size=49680
[11920427508] [INFO] [bran::requests] [CPU0]   [15] /boot/rtl8168d (cmdline='') size=45584
[11921062989] [INFO] [bran::requests] [CPU0]   [16] /boot/netd (cmdline='') size=147984
[11921657220] [INFO] [bran::requests] [CPU0]   [17] /boot/fetchd (cmdline='') size=41488
[11922260559] [INFO] [bran::requests] [CPU0]   [18] /boot/anther (cmdline='') size=586272
[11922895182] [INFO] [bran::requests] [CPU0]   [19] /boot/fortune (cmdline='') size=61968
[11923508223] [INFO] [bran::requests] [CPU0]   [20] /boot/photosynthesis (cmdline='') size=12648
[11924184888] [INFO] [bran::requests] [CPU0]   [21] /boot/ahci_disk (cmdline='') size=45664
[11924806245] [INFO] [bran::requests] [CPU0]   [22] /boot/iso9660d (cmdline='') size=53856
[11925422355] [INFO] [bran::requests] [CPU0]   [23] /boot/virtio_sound (cmdline='init') size=49648
[11926089912] [INFO] [bran::requests] [CPU0]   [24] /boot/hdaudio (cmdline='') size=37360
[11926698663] [INFO] [bran::requests] [CPU0]   [25] /boot/pci_stubd (cmdline='') size=29168
[11927333847] [INFO] [bran::requests] [CPU0]   [26] /boot/beeper (cmdline='init') size=33296
[11927965368] [INFO] [bran::requests] [CPU0]   [27] /boot/nectar (cmdline='init') size=176656
[11928599529] [INFO] [bran::requests] [CPU0]   [28] /boot/hello_std (cmdline='') size=16880
[11929264941] [INFO] [bran::requests] [CPU0]   [29] /boot/stdio_demo (cmdline='') size=16880
[11929900752] [INFO] [bran::requests] [CPU0]   [30] /boot/telnetd (cmdline='') size=57872
[11930523132] [INFO] [bran::requests] [CPU0]   [31] /assets/wallpapers/clouds.bmp (cmdline='') size=786486
[11931245535] [INFO] [bran::requests] [CPU0]   [32] /assets/wallpapers/flower.bmp (cmdline='') size=8386614
[11931968961] [INFO] [bran::requests] [CPU0]   [33] /assets/wallpapers/leather.bmp (cmdline='') size=1179702
[11932702617] [INFO] [bran::requests] [CPU0]   [34] /assets/wallpapers/linen.bmp (cmdline='') size=4718646
[11933417991] [INFO] [bran::requests] [CPU0]   [35] /assets/fonts/NotoSans-Regular.ttf (cmdline='') size=569208
[11934181347] [INFO] [bran::requests] [CPU0]   [36] /assets/themes/genie_circles.wasm (cmdline='') size=2824
[11934912924] [INFO] [bran::requests] [CPU0]   [37] /assets/cursors/future/default.svg (cmdline='') size=3051
[11935647438] [INFO] [bran::requests] [CPU0]   [38] /boot/locale.conf (cmdline='') size=85
[11936994960] [INFO] [kernel::memory] [CPU0] Memory map has 64 entries
[11938449534] [INFO] [kernel::memory] [CPU0]   [0] 0x0 - 0x87000 (Usable)
[11939258298] [INFO] [kernel::memory] [CPU0]   [1] 0x87000 - 0x88000 (Reserved)
[11939871768] [INFO] [kernel::memory] [CPU0]   [2] 0x88000 - 0xa0000 (Usable)
[11940473292] [INFO] [kernel::memory] [CPU0]   [3] 0x100000 - 0x800000 (Usable)
[11941182957] [INFO] [kernel::memory] [CPU0]   [4] 0x800000 - 0x808000 (Other)
[11941906680] [INFO] [kernel::memory] [CPU0]   [5] 0x808000 - 0x80b000 (Usable)
[11942593707] [INFO] [kernel::memory] [CPU0]   [6] 0x80b000 - 0x80c000 (Other)
[11943150087] [INFO] [kernel::memory] [CPU0]   [7] 0x80c000 - 0x811000 (Usable)
[11943714354] [INFO] [kernel::memory] [CPU0]   [8] 0x811000 - 0x900000 (Other)
[11944245093] [INFO] [kernel::memory] [CPU0]   [9] 0x900000 - 0x1780000 (Reserved)
[11944802562] [INFO] [kernel::memory] [CPU0]   [10] 0x1780000 - 0x7797d000 (Usable)
[11945366961] [INFO] [kernel::memory] [CPU0]   [11] 0x7797d000 - 0x779e1000 (Reserved)
[11945949015] [INFO] [kernel::memory] [CPU0]   [12] 0x779e1000 - 0x779e2000 (Other)
[11946509619] [INFO] [kernel::memory] [CPU0]   [13] 0x779e2000 - 0x779e3000 (Reserved)
[11947132956] [INFO] [kernel::memory] [CPU0]   [14] 0x779e3000 - 0x779e4000 (Other)
[11947749759] [INFO] [kernel::memory] [CPU0]   [15] 0x779e4000 - 0x779e5000 (Reserved)
[11948338446] [INFO] [kernel::memory] [CPU0]   [16] 0x779e5000 - 0x779e6000 (Other)
[11948906013] [INFO] [kernel::memory] [CPU0]   [17] 0x779e6000 - 0x779e7000 (Reserved)
[11949491862] [INFO] [kernel::memory] [CPU0]   [18] 0x779e7000 - 0x77a72000 (Other)
[11950057416] [INFO] [kernel::memory] [CPU0]   [19] 0x77a72000 - 0x77a73000 (Reserved)
[11950679169] [INFO] [kernel::memory] [CPU0]   [20] 0x77a73000 - 0x77ef4000 (Other)
[11951246736] [INFO] [kernel::memory] [CPU0]   [21] 0x77ef4000 - 0x77ef5000 (Reserved)
[11951833773] [INFO] [kernel::memory] [CPU0]   [22] 0x77ef5000 - 0x78016000 (Other)
[11952395367] [INFO] [kernel::memory] [CPU0]   [23] 0x78016000 - 0x78017000 (Reserved)
[11953016394] [INFO] [kernel::memory] [CPU0]   [24] 0x78017000 - 0x78817000 (Other)
[11953857795] [INFO] [kernel::memory] [CPU0]   [25] 0x78817000 - 0x78818000 (Reserved)
[11954889705] [INFO] [kernel::memory] [CPU0]   [26] 0x78818000 - 0x788d9000 (Other)
[11955496575] [INFO] [kernel::memory] [CPU0]   [27] 0x788d9000 - 0x788da000 (Reserved)
[11956077639] [INFO] [kernel::memory] [CPU0]   [28] 0x788da000 - 0x788e9000 (Other)
[11956639893] [INFO] [kernel::memory] [CPU0]   [29] 0x788e9000 - 0x788ea000 (Reserved)
[11957303952] [INFO] [kernel::memory] [CPU0]   [30] 0x788ea000 - 0x788ef000 (Other)
[11957873763] [INFO] [kernel::memory] [CPU0]   [31] 0x788ef000 - 0x788f0000 (Reserved)
[11958457335] [INFO] [kernel::memory] [CPU0]   [32] 0x788f0000 - 0x788f5000 (Other)
[11959021239] [INFO] [kernel::memory] [CPU0]   [33] 0x788f5000 - 0x788f6000 (Reserved)
[11959604052] [INFO] [kernel::memory] [CPU0]   [34] 0x788f6000 - 0x78922000 (Other)
[11960184555] [INFO] [kernel::memory] [CPU0]   [35] 0x78922000 - 0x78924000 (Reserved)
[11960766312] [INFO] [kernel::memory] [CPU0]   [36] 0x78924000 - 0x7892d000 (Other)
[11961329853] [INFO] [kernel::memory] [CPU0]   [37] 0x7892d000 - 0x7892f000 (Reserved)
[11961914514] [INFO] [kernel::memory] [CPU0]   [38] 0x7892f000 - 0x78937000 (Other)
[11962479771] [INFO] [kernel::memory] [CPU0]   [39] 0x78937000 - 0x78938000 (Reserved)
[11963061561] [INFO] [kernel::memory] [CPU0]   [40] 0x78938000 - 0x78942000 (Other)
[11963636817] [INFO] [kernel::memory] [CPU0]   [41] 0x78942000 - 0x78943000 (Reserved)
[11964220059] [INFO] [kernel::memory] [CPU0]   [42] 0x78943000 - 0x78950000 (Other)
[11964782610] [INFO] [kernel::memory] [CPU0]   [43] 0x78950000 - 0x78952000 (Reserved)
[11965365225] [INFO] [kernel::memory] [CPU0]   [44] 0x78952000 - 0x78960000 (Other)
[11965928799] [INFO] [kernel::memory] [CPU0]   [45] 0x78960000 - 0x78961000 (Reserved)
[11966540454] [INFO] [kernel::memory] [CPU0]   [46] 0x78961000 - 0x7896d000 (Other)
[11967152340] [INFO] [kernel::memory] [CPU0]   [47] 0x7896d000 - 0x7896e000 (Reserved)
[11967815673] [INFO] [kernel::memory] [CPU0]   [48] 0x7896e000 - 0x78972000 (Other)
[11968559955] [INFO] [kernel::memory] [CPU0]   [49] 0x78972000 - 0x78973000 (Reserved)
[11969214939] [INFO] [kernel::memory] [CPU0]   [50] 0x78973000 - 0x78983000 (Other)
[11969781120] [INFO] [kernel::memory] [CPU0]   [51] 0x78983000 - 0x78984000 (Reserved)
[11970385053] [INFO] [kernel::memory] [CPU0]   [52] 0x78984000 - 0x78a14000 (Other)
[11970947769] [INFO] [kernel::memory] [CPU0]   [53] 0x78a14000 - 0x78a15000 (Reserved)
[11971529988] [INFO] [kernel::memory] [CPU0]   [54] 0x78a15000 - 0x78a20000 (Other)
[11972093001] [INFO] [kernel::memory] [CPU0]   [55] 0x78a20000 - 0x78a21000 (Reserved)
[11972674032] [INFO] [kernel::memory] [CPU0]   [56] 0x78a21000 - 0x78a46000 (Other)
[11973236715] [INFO] [kernel::memory] [CPU0]   [57] 0x78a46000 - 0x78a47000 (Reserved)
[11973829527] [INFO] [kernel::memory] [CPU0]   [58] 0x78a47000 - 0x78a53000 (Other)
[11974391583] [INFO] [kernel::memory] [CPU0]   [59] 0x78a53000 - 0x78a54000 (Reserved)
[11975039538] [INFO] [kernel::memory] [CPU0]   [60] 0x78a54000 - 0x78a61000 (Other)
[11975602155] [INFO] [kernel::memory] [CPU0]   [61] 0x78a61000 - 0x78a62000 (Reserved)
[11976183186] [INFO] [kernel::memory] [CPU0]   [62] 0x78a62000 - 0x78aaa000 (Other)
[11976759201] [INFO] [kernel::memory] [CPU0]   [63] 0x78aaa000 - 0x78aab000 (Reserved)
[11977609479] [INFO] [kernel::memory] [CPU0] HHDM Offset: 0xffff800000000000
[12227622924] [CONTRACT] [kernel::memory] [CPU0] Frame allocator initialized with 485781 free frames
[12239104350] [INFO] [bran::arch] [CPU0] IOAPIC: hhdm=0xffff800000000000
[12244559019] [INFO] [bran::arch] [CPU0] IOAPIC: Disabling legacy PIC...
[12245920929] [INFO] [bran::arch] [CPU0] IOAPIC: PIC disabled OK
[12246847437] [INFO] [bran::arch] [CPU0] IOAPIC: RSDP virt=0x7f77e014
[12251219310] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: total length 144
[12252956529] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77802c
[12254236335] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778034
[12254954283] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77803c
[12255652695] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778044
[12256342692] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 1, len 12 at 0xffff80007f77804c
[12257359620] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778058
[12258359025] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778062
[12259070604] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f77806c
[12259768059] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778076
[12260490561] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778080
[12261188643] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 4, len 6 at 0xffff80007f77808a
[12262405749] [INFO] [bran::arch] [CPU0] IOAPIC: MADT parsed OK
[12263581341] [INFO] [bran::arch] [CPU0] SMP: Found 4 CPUs (CPU_COUNT now = 4)
[12264392382] [INFO] [bran::arch] [CPU0] IOAPIC: Found at phys 0xfec00000, GSI base 0
[12266020371] [INFO] [bran::arch] [CPU0] IOAPIC: Registers initialized
[12267016674] [INFO] [bran::arch] [CPU0] IOAPIC: Local APIC enabled (SVR=0x1FF, TPR=0)
[12268042644] [INFO] [bran::arch] [CPU0] IOAPIC: version 0x20, 24 redir entries
[12269954664] [INFO] [bran::arch] [CPU0] IOAPIC: All pins masked
[12271788606] [INFO] [bran::arch] [CPU0] IOAPIC: IRQ1 -> GSI 1 -> 0x21
[12272661357] [INFO] [bran::arch] [CPU0] IOAPIC: IRQ12 -> GSI 12 -> 0x2C
[12273254037] [INFO] [bran::arch] [CPU0] IOAPIC: Init complete
[12274051680] [CONTRACT] [kernel] [CPU0] Initializing global allocator...
[12669890541] [INFO] [kernel::memory::global_alloc] [CPU0] Global allocator initialized (LinkedHeap, 32MB)
[12670922319] [CONTRACT] [kernel] [CPU0] Seeding entropy pool...
[12674537304] [INFO] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (NOT seeded)
[12675480345] [INFO] [kernel::entropy] [CPU0] ENTROPY: marked seeded with weak entropy (timer-only fallback)
[12676276767] [CONTRACT] [kernel] [CPU0] Initializing SIMD...
[12677832189] [CONTRACT] [kernel] [CPU0] Initializing tasking...
[12690491451] [INFO] [kernel::sched] [CPU0]   Acquiring scheduler lock...
[12691881675] [INFO] [kernel::sched] [CPU0]   Lock acquired, checking if initialized...
[12692669286] [INFO] [kernel::sched] [CPU0]   Allocating scheduler...
[12694565829] [INFO] [kernel::sched] [CPU0]   Leaking scheduler...
[12695204181] [INFO] [kernel::sched] [CPU0]   Initializing boot task...
[12697829694] [INFO] [kernel::sched] [CPU0]   Creating boot task...
[12705742764] [INFO] [kernel::sched] [CPU0]   Creating idle tasks...
[12707589840] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 1 assigned to CPU 0
[12721967709] [INFO] [kernel::sched] [CPU0]   Boot task initialized
[12722583126] [INFO] [kernel::sched] [CPU0]   Storing scheduler pointer...
[12740516019] [INFO] [kernel::sched] [CPU0]   Initialized 4 event ring(s)
[12741191694] [CONTRACT] [kernel::sched] [CPU0] Scheduler initialized
[12743294289] [CONTRACT] [alloc] [CPU0] MEDIUM ALLOC #1: 224 KB align=8 total=0MB
[12744571422] [INFO] [kernel::task] [CPU0]   Creating graph worker tasks...
[12745654878] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 2 assigned to CPU 0
[12748327746] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 3 assigned to CPU 0
[12749615472] [INFO] [kernel] [CPU0] System initialized. Setting up preemption timer (100Hz)...
[12786043083] [INFO] [bran::arch::x86_64] [CPU0] LAPIC: calibrated timer (62371300 ticks/sec), init_cnt=623713 for 100Hz
[12788390439] [INFO] [kernel] [CPU0] Kernel: Detected 4 CPUs. Starting 3 secondaries...
[12789677703] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting 3 secondary CPUs...
[12791572167] [INFO] [bran::arch::x86_64] [CPU0] SMP: Initializing trampoline at 0x8000
[12800647101] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 1 (APIC 1)
[12837008976] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 1 (APIC 1) is online
[12838597530] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 2 (APIC 2)
[12839911062] [INFO] [kernel] [CPU1] SMP: Entering kernel_secondary_entry for CPU 1
[12841945314] [INFO] [kernel::sched] [CPU1] SMP: CPU 1 online (triggered by scheduler spawn)
[12843844860] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 4 assigned to CPU 1
[12848656095] [INFO] [kernel::sched] [CPU1] SMP: Secondary CPU 1 online!
[12850866336] [INFO] [kernel::task] [CPU1] SMP: CPU 1 bootstrapped with idle task 4
[12873751836] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 2 (APIC 2) is online
[12875048241] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 3 (APIC 3)
[12876392265] [INFO] [kernel] [CPU2] SMP: Entering kernel_secondary_entry for CPU 2
[12878061438] [INFO] [kernel::sched] [CPU2] SMP: CPU 2 online (triggered by scheduler spawn)
[12879637188] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 5 assigned to CPU 2
[12881412852] [INFO] [kernel::sched] [CPU2] SMP: Secondary CPU 2 online!
[12882581481] [INFO] [kernel::task] [CPU2] SMP: CPU 2 bootstrapped with idle task 5
[12910394871] [INFO] [kernel] [CPU3] SMP: Entering kernel_secondary_entry for CPU 3
[12911627058] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 3 (APIC 3) is online
[12913256235] [INFO] [kernel::sched] [CPU3] SMP: CPU 3 online (triggered by scheduler spawn)
[12914311608] [INFO] [bran::arch::x86_64] [CPU0] SMP: Secondary CPU startup complete
[12915672990] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 6 assigned to CPU 3
[12916760175] [INFO] [kernel] [CPU0] Kernel: Secondary CPU bring-up complete.
[12917991603] [INFO] [kernel::sched] [CPU3] SMP: Secondary CPU 3 online!
[12918763275] [INFO] [kernel::task] [CPU3] SMP: CPU 3 bootstrapped with idle task 6
[12927088185] [INFO] [kernel::root] [CPU0] Spawning Root service...
[12928301595] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 7 assigned to CPU 1
[12930778806] [INFO] [kernel::root::boot_register] [CPU0] ROOT: boot registration begin (Census Phase 1 v0.2)
[12932093955] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 1. Host creation start
[12937979010] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
RM
[12940380783] [INFO] [kernel::root::service] [CPU1] ROOT: started once
[12941140707] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Initializing components...
[12942598119] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Graph initialized
[12944163705] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Journal initialized
[12945578415] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Interner initialized
[12966219222] [CONTRACT] [kernel::root::service] [CPU1] ROOT: LogSymbols initialized
[12971458203] [CONTRACT] [kernel::root::service] [CPU1] ROOT: BatchScratch initialized
[12973369728] [CONTRACT] [kernel::root::service] [CPU1] ROOT: QueryScratch initialized
[12974649732] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Entering main loop
[13040282112] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13046223300] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13048772583] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13052618700] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13056476697] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13059596946] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 1. Host registered: t1
[13060901766] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 2. Platform Bus start
[13062634134] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13075301778] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13078665963] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13083520560] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13087353180] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13091108811] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13095145536] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 2. Platform Bus done
[13096326903] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 3. Kernel start
[13098074682] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[13118956785] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 3. Kernel done
[13120164651] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 4. Root Service start
[13132324755] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 4. Root Service done
[13133602779] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 5. CPUs start
[13177592538] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 5. CPUs done
[13178428428] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 6. Memory Ranges start
[13618524942] [CONTRACT] [alloc] [CPU1] MEDIUM ALLOC #2: 120 KB align=8 total=1MB
[14134914354] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 6. Memory Ranges done
[14166608511] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=1000 nodes=331 watches=0 history=499 journal=424 symbols=51 drops=0
[14199686622] [CONTRACT] [alloc] [CPU1] MEDIUM ALLOC #3: 240 KB align=8 total=1MB
[15809455365] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=2000 nodes=450 watches=0 history=966 journal=774 symbols=98 drops=0
[16630180908] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: filesystem bootstrap: /initrd/ with 39 files
[16733845062] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: Census Phase 2: PCI
[16735001118] [INFO] [kernel::root::pci] [CPU0] PCI: Starting enumeration...
[16844185545] [INFO] [kernel::root::pci] [CPU0] PCI: 00:00.0 8086:29c0 Intel Corporation 82G33/G31/P35/P31 Express DRAM Controller class=06:00 prog_if=00 rev=00
[16905695334] [INFO] [kernel::root::pci] [CPU0] PCI: 00:01.0 1af4:1050 Red Hat, Inc. Virtio 1.0 GPU class=03:00 prog_if=00 rev=01
[16934315178] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR0: phys=0x80000000 size=0x800000
[16935329400] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR2: phys=0xc000000000 size=0x4000
[16935994251] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR4: phys=0x80882000 size=0x1000
[16940033253] [INFO] [kernel::root::pci] [CPU0] PCI: Found virtio display controller at 00:01.0
[16959346734] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO notify_cfg BAR2 offset=0x3000 mult=4
[16987185996] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO common_cfg BAR2 offset=0x1000
[16989609417] [INFO] [kernel::root::pci] [CPU0] PCI: Registered virtio GPU (graph_id=1086, idx=1) BAR0=0x80000000
[17077560357] [INFO] [kernel::root::pci] [CPU0] PCI: 00:02.0 1af4:1000 Red Hat, Inc. Virtio network device class=02:00 prog_if=00 rev=00
[17102972073] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR1: phys=0x80881000 size=0x1000
[17104174494] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR4: phys=0xc000004000 size=0x4000
[17110751526] [INFO] [kernel::root::pci] [CPU0] PCI: Found virtio network controller at 00:02.0
[17142893790] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO notify_cfg BAR4 offset=0x3000 mult=4
[17165451996] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO common_cfg BAR4 offset=0x0
[17168843241] [INFO] [kernel::root::pci] [CPU0] PCI: Registered virtio network (graph_id=1120, idx=2) BAR4=0xc000004000
[17169843075] [INFO] [kernel::root::pci] [CPU0] VirtIO-Net: Device registered for userspace driver
[17242188348] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.0 8086:2918 Intel Corporation 82801IB (ICH9) LPC Interface Controller class=06:01 prog_if=00 rev=02
[17243908275] [INFO] [kernel::root::pci] [CPU0] PCI: Found LPC/ISA bridge at 00:1f.0
[17339870064] [INFO] [kernel::root::pci] [CPU0] LPC: Created Legacy IO bus with CMOS and PS/2 controller
[17409427266] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.2 8086:2922 Intel Corporation 82801IR/IO/IH (ICH9R/DO/DH) 6 port SATA Controller [AHCI mode] class=01:06 prog_if=01 rev=02
[17421203910] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR5: phys=0x80880000 size=0x1000
[17424960300] [INFO] [kernel::root::pci] [CPU0] PCI: Found AHCI SATA controller at 00:1f.2
[17427199416] [INFO] [kernel::root::pci] [CPU0] PCI: Registered AHCI controller (graph_id=1193, idx=5) BAR5=0x80880000
[17470944711] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=3000 nodes=584 watches=0 history=1024 journal=1024 symbols=181 drops=0
[17498133048] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.3 8086:2930 Intel Corporation 82801I (ICH9 Family) SMBus Controller class=0c:05 prog_if=00 rev=02
[17503156473] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: registered items. host=t1 kernel=tb
[17504184192] [CONTRACT] [kernel] [CPU0] KERNEL: root census complete: host=t1 kernel=tb root=te
[17505035823] [CONTRACT] [kernel] [CPU0] Kernel: Enumerating 39 boot modules...
[17505938835] [CONTRACT] [kernel] [CPU0]   [0] name='/boot/sprout' cmdline='init' size=111120
[17506653417] [CONTRACT] [kernel] [CPU0]   [1] name='/boot/bristle' cmdline='init' size=41456
[17507480298] [CONTRACT] [kernel] [CPU0]   [2] name='/boot/rtc_cmos' cmdline='init' size=29248
[17508177621] [CONTRACT] [kernel] [CPU0]   [3] name='/boot/clock' cmdline='' size=61968
[17508972195] [CONTRACT] [kernel] [CPU0]   [4] name='/boot/taskman' cmdline='' size=70160
[17509611240] [CONTRACT] [kernel] [CPU0]   [5] name='/boot/ps2_kbd' cmdline='init' size=25072
[17510351661] [CONTRACT] [kernel] [CPU0]   [6] name='/boot/echo' cmdline='init' size=25072
[17510998164] [CONTRACT] [kernel] [CPU0]   [7] name='/boot/bloom' cmdline='init' size=965128
[17511653148] [CONTRACT] [kernel] [CPU0]   [8] name='/boot/ps2_mouse' cmdline='init' size=29168
[17512461582] [CONTRACT] [kernel] [CPU0]   [9] name='/boot/display_bootfb' cmdline='init' size=29248
[17513415480] [CONTRACT] [kernel] [CPU0]   [10] name='/boot/display_virtio_gpu' cmdline='init' size=53864
[17514324036] [CONTRACT] [kernel] [CPU0]   [11] name='/boot/fontd' cmdline='init' size=188944
[17515066206] [CONTRACT] [kernel] [CPU0]   [12] name='/boot/blossom' cmdline='init' size=123408
[17515743630] [CONTRACT] [kernel] [CPU0]   [13] name='/boot/flytrap' cmdline='' size=291352
[17516391420] [CONTRACT] [kernel] [CPU0]   [14] name='/boot/virtio_netd' cmdline='' size=49680
[17517082605] [CONTRACT] [kernel] [CPU0]   [15] name='/boot/rtl8168d' cmdline='' size=45584
[17517732606] [CONTRACT] [kernel] [CPU0]   [16] name='/boot/netd' cmdline='' size=147984
[17518359606] [CONTRACT] [kernel] [CPU0]   [17] name='/boot/fetchd' cmdline='' size=41488
[17518993668] [CONTRACT] [kernel] [CPU0]   [18] name='/boot/anther' cmdline='' size=586272
[17519681223] [CONTRACT] [kernel] [CPU0]   [19] name='/boot/fortune' cmdline='' size=61968
[17520344292] [CONTRACT] [kernel] [CPU0]   [20] name='/boot/photosynthesis' cmdline='' size=12648
[17521032705] [CONTRACT] [kernel] [CPU0]   [21] name='/boot/ahci_disk' cmdline='' size=45664
[17521686633] [CONTRACT] [kernel] [CPU0]   [22] name='/boot/iso9660d' cmdline='' size=53856
[17522418111] [CONTRACT] [kernel] [CPU0]   [23] name='/boot/virtio_sound' cmdline='init' size=49648
[17523127083] [CONTRACT] [kernel] [CPU0]   [24] name='/boot/hdaudio' cmdline='' size=37360
[17523871398] [CONTRACT] [kernel] [CPU0]   [25] name='/boot/pci_stubd' cmdline='' size=29168
[17524543575] [CONTRACT] [kernel] [CPU0]   [26] name='/boot/beeper' cmdline='init' size=33296
[17525206677] [CONTRACT] [kernel] [CPU0]   [27] name='/boot/nectar' cmdline='init' size=176656
[17525874729] [CONTRACT] [kernel] [CPU0]   [28] name='/boot/hello_std' cmdline='' size=16880
[17526527337] [CONTRACT] [kernel] [CPU0]   [29] name='/boot/stdio_demo' cmdline='' size=16880
[17527206378] [CONTRACT] [kernel] [CPU0]   [30] name='/boot/telnetd' cmdline='' size=57872
[17527846050] [CONTRACT] [kernel] [CPU0]   [31] name='/assets/wallpapers/clouds.bmp' cmdline='' size=786486
[17528601189] [CONTRACT] [kernel] [CPU0]   [32] name='/assets/wallpapers/flower.bmp' cmdline='' size=8386614
[17529363423] [CONTRACT] [kernel] [CPU0]   [33] name='/assets/wallpapers/leather.bmp' cmdline='' size=1179702
[17530156644] [CONTRACT] [kernel] [CPU0]   [34] name='/assets/wallpapers/linen.bmp' cmdline='' size=4718646
[17530912146] [CONTRACT] [kernel] [CPU0]   [35] name='/assets/fonts/NotoSans-Regular.ttf' cmdline='' size=569208
[17531700450] [CONTRACT] [kernel] [CPU0]   [36] name='/assets/themes/genie_circles.wasm' cmdline='' size=2824
[17532465291] [CONTRACT] [kernel] [CPU0]   [37] name='/assets/cursors/future/default.svg' cmdline='' size=3051
[17533258050] [CONTRACT] [kernel] [CPU0]   [38] name='/boot/locale.conf' cmdline='' size=85
[17534807598] [INFO] [kernel] [CPU0] Found init module: /boot/sprout (cmdline: 'init'), loading...
[17536650681] [INFO] [kernel::task::loader] [CPU0] Loading module: /boot/sprout
[17537512311] [DEBUG] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[17546340735] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[17561121435] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=213000 exec=false
[17565619203] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=21a000 exec=false
[17575546593] [INFO] [kernel] [CPU0] Spawning sprout with registry at 0x600000...
[17576320938] [CONTRACT] [kernel] [CPU0] Spawning init process...
[17578733700] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 8 (user task/process) assigned to CPU 2
[17582046504] [CONTRACT] [kernel] [CPU0] Entering scheduler loop.
[17583026340] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600000
ThingOS Petals
type 'help' for commands

petals> [17590515096] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=8 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369013568 RFLAGS_BEFORE=130 CR3_BEFORE=50393088 fs_base=0 gs_base=18446744071564586608
[17595501858] [DEBUG] [kernel::sched::sleep] [CPU0] DIAG ctx_switch: cpu=0 from=0 to=2
[17611611072] [INFO] [sprout] [CPU2] [sprout] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ff990 rip=0x20139f rflags=0x206
[17616667596] [INFO] [sprout] [CPU2] SPROUT: v0.4 starting (Supervisor Mode)...
[17617987101] [INFO] [sprout::devtree] [CPU2] SPROUT: devtree::init entry (v0.2)
[17619198894] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 1: Find Host
[17627977884] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 2: HHDM
[17633394306] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 3: Platform Bus
[17637560094] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 4: Firmware
[17638896396] [INFO] [sprout::devtree] [CPU2] SPROUT: Finding ACPI...
[17643814353] [INFO] [sprout::devtree] [CPU2] SPROUT: Found 1 ACPI nodes
[17647912326] [INFO] [sprout::devtree] [CPU2] SPROUT: ACPI RSDP = 0x7f77e014
[17649164214] [INFO] [sprout::devtree] [CPU2] SPROUT: Finding DTB...
[17653807446] [INFO] [sprout::devtree] [CPU2] SPROUT: Found 0 DTB nodes
[17655018249] [INFO] [sprout::devtree] [CPU2] SPROUT: Init OK, returning context
[17656308516] [INFO] [sprout::devtree] [CPU2] SPROUT: build() called
[17657521398] [INFO] [sprout::devtree::x86_64] [CPU2] SPROUT: x86_64 platform enrichment... (v0.2)
[17661531294] [INFO] [sprout::devtree::x86_64] [CPU2] SPROUT: x86_64 enumerate done
[17662866276] [INFO] [sprout] [CPU2] SPROUT: About to create Supervisor...
[17664228879] [INFO] [sprout] [CPU2] SPROUT: Supervisor created, calling run_forever...
[17665810338] [INFO] [sprout::supervisor] [CPU2] SPROUT: Supervisor starting (phased mode)...
[17667323223] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 1] Hardware Discovery and Core Drivers
[17668856040] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovering modules...
[17677879560] [INFO] [sprout::supervisor] [CPU2] SPROUT: Found 39 modules
[17720499786] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[0] = '/boot/sprout'
[17729230629] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[1] = '/boot/bristle'
[17735986620] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[2] = '/boot/rtc_cmos'
[17743932096] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[3] = '/boot/clock'
[17747997696] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/clock
[17753493582] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[4] = '/boot/taskman'
[17759913237] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[5] = '/boot/ps2_kbd'
[17766783243] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[6] = '/boot/echo'
[17774925432] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[7] = '/boot/bloom'
[17780705415] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[8] = '/boot/ps2_mouse'
[17787303534] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[9] = '/boot/display_bootfb'
[17793618084] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[10] = '/boot/display_virtio_gpu'
[17799916893] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[11] = '/boot/fontd'
[17805502044] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[12] = '/boot/blossom'
[17811088020] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[13] = '/boot/flytrap'
[17817265950] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[14] = '/boot/virtio_netd'
[17824934127] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[15] = '/boot/rtl8168d'
[17831993091] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[16] = '/boot/netd'
[17838391956] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[17] = '/boot/fetchd'
[17846033502] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[18] = '/boot/anther'
[17853133122] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[19] = '/boot/fortune'
[17860072065] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[20] = '/boot/photosynthesis'
[17866435851] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[21] = '/boot/ahci_disk'
[17875443630] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[22] = '/boot/iso9660d'
[17885317065] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[23] = '/boot/virtio_sound'
[17893839579] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[24] = '/boot/hdaudio'
[17901670248] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[25] = '/boot/pci_stubd'
[17910990108] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[26] = '/boot/beeper'
[17918128734] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[27] = '/boot/nectar'
[17926218618] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[28] = '/boot/hello_std'
[17932021965] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/hello_std
[17936879004] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[29] = '/boot/stdio_demo'
[17942681955] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[30] = '/boot/telnetd'
[17948723397] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[31] = '/assets/wallpapers/clouds.bmp'
[17956030587] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[32] = '/assets/wallpapers/flower.bmp'
[17964053184] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[33] = '/assets/wallpapers/leather.bmp'
[17972531577] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[34] = '/assets/wallpapers/linen.bmp'
[17981504937] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[35] = '/assets/fonts/NotoSans-Regular.ttf'
[17990479188] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[36] = '/assets/themes/genie_circles.wasm'
[17998672395] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[37] = '/assets/cursors/future/default.svg'
[18006720501] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[38] = '/boot/locale.conf'
[18011425113] [INFO] [sprout::registry] [CPU2] SPROUT: Scanning boot modules...
[18039133728] [INFO] [sprout::registry] [CPU2] SPROUT: Registering driver 'dev.rtc.Cmos' -> '/boot/rtc_cmos' (fallback)
[18231218214] [INFO] [sprout::registry] [CPU2] SPROUT: Registry scan complete. Found 1 drivers.
[18242670435] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/pci_stubd
[18244144314] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18247202688] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18254482422] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[18256527168] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[18270117591] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 9 (user task/process) assigned to CPU 3
[18278431182] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db400
[18279896646] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned pci_stubd (PID=9)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18282052668] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=9 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369079104 RFLAGS_BEFORE=130 CR3_BEFORE=50860032 fs_base=0 gs_base=18446744071564586640
[18287747313] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/rtc_cmos
[18288860931] [INFO] [pci_stubd] [CPU3] pci_stubd: starting pci-id matcher
[18289922970] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[18292729092] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18299396247] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=204000 exec=false
[18302211873] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[18313281987] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 10 (user task/process) assigned to CPU 1
[18318203079] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db400
[18319819419] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned rtc_cmos (PID=10)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x49b
[18321998376] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=10 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369144640 RFLAGS_BEFORE=130 CR3_BEFORE=50974720 fs_base=0 gs_base=18446744071564586576
[18326559372] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up storage pipeline...
[18331316520] [INFO] [rtc_cmos] [CPU1] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ffeb0 rip=0x201027 rflags=0x202
[18334238538] [INFO] [rtc_cmos] [CPU1] Starting... arg=49b
[18337675653] [INFO] [rtc_cmos] [CPU1] Serving device ID: ThingId([155, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[18344739732] [INFO] [rtc_cmos] [CPU1] RTC: 2026-04-06 00:55:19 = 1775436919 unix_secs
[18347566776] [INFO] [kernel::time] [CPU1] System clock anchored: unix_secs=1775436919, mono_ns=9173414844, offset=1775436909826585156ns
[18350585979] [INFO] [rtc_cmos] [CPU1] System clock anchored
[18367350606] [INFO] [rtc_cmos] [CPU1] RTC: Set sys.TimeState = 1 (Anchored)
[18421615773] [INFO] [rtc_cmos] [CPU1] Publishing time. Entering maintenance loop.
[18438143757] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ahci_disk
[18439406304] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[18441975783] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18450796848] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[18454337385] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20a000 exec=false
[18466612362] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 11 (user task/process) assigned to CPU 2
[18471559227] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ahci_disk (PID=11)
[18476677626] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18478908129] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=11 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369210688 RFLAGS_BEFORE=130 CR3_BEFORE=51089408 fs_base=0 gs_base=18446744071564586608
[18485914986] [INFO] [ahci_disk] [CPU2] AHCI: Starting AHCI/SATA disk driver v1
[18492127203] [INFO] [sprout::pipelines] [CPU2] SPROUT: Failed to spawn ata_disk: ENOENT
[18494292168] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/iso9660d
[18495374799] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[18498118254] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[18509083098] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[18512443719] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[18522869970] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 12 (user task/process) assigned to CPU 3
[18528345561] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned iso9660d (PID=12)
[18529498680] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[18531717798] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=12 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369277440 RFLAGS_BEFORE=134 CR3_BEFORE=51228672 fs_base=0 gs_base=18446744071564586640
[18534975261] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up audio driver...
[18537195105] [INFO] [iso9660d] [CPU3] ISO9660D: Starting ISO9660 mount service...
[18543111378] [INFO] [ahci_disk] [CPU2] AHCI: Found 6 PCI functions
[18544158864] [INFO] [iso9660d] [CPU3] ISO9660D: Found 0 block devices
[18545729136] [INFO] [iso9660d] [CPU3] ISO9660D: No ISO9660 filesystems found
[18547640661] [INFO] [iso9660d] [CPU3] ISO9660D: Entering service loop with 0 mounts
[18556590492] [INFO] [sprout::pipelines] [CPU2] SPROUT: No Sound device found
[18558734469] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up display pipeline...
[18580217337] [INFO] [sprout::pipelines] [CPU2] SPROUT: Using VirtIO GPU at 1920x1080
[18582699465] [INFO] [sprout::pipelines] [CPU2] SPROUT: Display backend: VirtIO-GPU (1920x1080 stride=7680)
[19087469346] [INFO] [ahci_disk] [CPU2] AHCI: Found AHCI controller at PCI func ThingId([169, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[19098640044] [INFO] [ahci_disk] [CPU2] AHCI: BAR5=0x80880000
[19101417258] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 11 claimed device 1193 (handle 0)
[19103075409] [INFO] [ahci_disk] [CPU2] AHCI: Claimed PCI device ThingId([169, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) handle=0
[19108112067] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR5 phys=0x80880000 size=0x1000 -> virt=0x10000000
[19109658843] [INFO] [ahci_disk] [CPU2] AHCI: Mapped ABAR at 0x10000000
[19111662372] [INFO] [ahci_disk] [CPU2] AHCI: Version 1.0, 6 ports, 32 slots, 64-bit: true
[19113047283] [INFO] [ahci_disk] [CPU2] AHCI: Ports implemented: 0x3f
[19114016163] [INFO] [ahci_disk] [CPU2] AHCI: DMA virt=0x20b000
[19116702957] [INFO] [ahci_disk] [CPU2] AHCI: DMA phys=0x30c8000
[19117809018] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 0...
[19119018633] [INFO] [ahci_disk] [CPU2] AHCI: Port 0 - no device
[19120009326] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 1...
[19120831158] [INFO] [ahci_disk] [CPU2] AHCI: Port 1 - no device
[19121678928] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 2...
[19122611871] [INFO] [ahci_disk] [CPU2] AHCI: Port 2 - SATAPI drive (sig=0xeb140101)
[19131685386] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/display_virtio_gpu
[19132666872] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[19134624234] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19141728606] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[19144266339] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[19151465355] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 13 (user task/process) assigned to CPU 1
[19170269976] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00fab68
[19171857540] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned display driver '/display_virtio_gpu' (PID=13)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x30002
[19173894102] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=13 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369357152 RFLAGS_BEFORE=134 CR3_BEFORE=59666432 fs_base=0 gs_base=18446744071564586576
[19181898483] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: starting (drv_req_r=2, drv_resp_w=3)
[19190042190] [INFO] [kernel::syscall::handlers::device] [CPU1] DEVICE: task 13 claimed device 1108 (handle 1)
[19201205397] [INFO] [ahci_disk] [CPU2] AHCI: Registered ATAPI block device 1235 port=2 model='                                        ' rpc_port=5
[19211293992] [INFO] [virtio_gpu] [CPU1] virtio_gpu: caps from graph - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[19213312074] [INFO] [kernel::syscall::handlers::device] [CPU1] DEVICE: Mapped BAR2 phys=0xc000000000 size=0x4000 -> virt=0x10001000
[19218767106] [INFO] [virtio_gpu] [CPU1] virtio_gpu: device features=0x30000002 virgl=false
[19227766767] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: GPU initialized successfully
[19229475144] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: Virgl 3D not supported, using 2D only
[19234233084] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 3...
[19235233248] [INFO] [ahci_disk] [CPU2] AHCI: Port 3 - no device
[19236161175] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 4...
[19237201896] [INFO] [ahci_disk] [CPU2] AHCI: Port 4 - no device
[19238167476] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 5...
[19239066000] [INFO] [ahci_disk] [CPU2] AHCI: Port 5 - no device
[19240458435] [INFO] [ahci_disk] [CPU2] AHCI: Found 1 SATA disk(s)
[19241642310] [INFO] [ahci_disk] [CPU2] AHCI: Entering RPC service loop
[19243304850] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: creating frame pool 1x 1920x1080 stride=7680 format=2
[19691697267] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up input pipeline (keyboard + mouse)...
[19695881667] [INFO] [sprout::pipelines] [CPU2] SPROUT: Created kbd_raw port (w=7, r=8)
[19700319210] [INFO] [sprout::pipelines] [CPU2] SPROUT: Created mouse_raw port (w=9, r=10)
[19702866282] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ps2_kbd
[19704124275] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19706724807] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19712206272] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=203000 exec=false
[19714876236] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[19726785507] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 14 (user task/process) assigned to CPU 2
[19732060392] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ps2_kbd (PID=14)
[19735655445] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ps2_mouse
[19737221955] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19740199578] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19747547985] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[19749683052] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[19760436234] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 15 (user task/process) assigned to CPU 3
[19765452894] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb0106680
[19766480712] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ps2_mouse (PID=15)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x9
[19768319175] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=15 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369500880 RFLAGS_BEFORE=130 CR3_BEFORE=68513792 fs_base=0 gs_base=18446744071564586640
[19775568087] [INFO] [ps2_mouse] [CPU3] ps2_mouse: online (handle=9)
[19776778461] [INFO] [ps2_mouse] [CPU3] ps2_mouse: enabling aux port
[19781969493] [INFO] [ps2_mouse] [CPU3] ps2_mouse: controller cfg already correct (0x47)
[19783395093] [INFO] [ps2_mouse] [CPU3] ps2_mouse: sending RESET (0xFF)
[19784114295] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/bristle
[19785453633] [INFO] [ps2_mouse] [CPU3] ps2_mouse: reset ACK received (0xfa)
[19786266258] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19788306318] [INFO] [ps2_mouse] [CPU3] ps2_mouse: BAT passed (0xaa), ID 0x00 confirmed
[19789253847] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19790490720] [INFO] [ps2_mouse] [CPU3] ps2_mouse: setting sample rate (100)
[19791943413] [INFO] [ps2_mouse] [CPU3] ps2_mouse: setting resolution (3)
[19793848536] [INFO] [ps2_mouse] [CPU3] ps2_mouse: status result = Some(0) Some(3) Some(100)
[19795207839] [INFO] [ps2_mouse] [CPU3] ps2_mouse: sending enable command (0xF4)
[19796523846] [INFO] [ps2_mouse] [CPU3] ps2_mouse: enable ACK received (0xfa)
[19797361320] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[19801174074] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[19807633494] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: frame pool ready (1 buffer)
[19811510235] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 16 (user task/process) assigned to CPU 1
[19816506732] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned bristle (PID=16)
[19819054266] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb0106680
[19820218539] [INFO] [sprout::pipelines] [CPU2] SPROUT: Input broker ready (keyboard + mouse)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x8000a000b000d
[19822251933] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=16 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369583824 RFLAGS_BEFORE=130 CR3_BEFORE=68894720 fs_base=0 gs_base=18446744071564586576
[19825657104] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up network stack...
[19827830187] [INFO] [bristle] [CPU1] BRISTLE_MAIN_ENTERED_WITH_LOGS_YAY
[19829115570] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00fab68
[19830634428] [INFO] [bristle] [CPU1] bristle: online (kbd=8, mouse=10, evt=11, echo=13)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x7
[19832458074] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=14 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369435088 RFLAGS_BEFORE=130 CR3_BEFORE=68390912 fs_base=0 gs_base=18446744071564586608
[19837045239] [INFO] [bristle] [CPU1] bristle: created broadcast topic 0
[19840536507] [INFO] [ps2_kbd] [CPU2] ps2_kbd: online (handle=7)
[19851521151] [INFO] [ps2_kbd] [CPU2] ps2_kbd: created driver node 1241
[19854877020] [INFO] [bristle] [CPU1] bristle: registered in graph as svc.Input (id=1240)
[19856118216] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task subscribed to vector 0x21
[19859012085] [INFO] [ps2_kbd] [CPU2] ps2_kbd: subscribed to IRQ1 (vector 0x21)
[19866673266] [INFO] [sprout::pipelines] [CPU2] SPROUT: Found NIC device ThingId([117, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[19869395469] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/virtio_netd
[19870254888] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19871961252] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19880799708] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=208000 exec=false
[19884099873] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20b000 exec=false
[19893297600] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 17 (user task/process) assigned to CPU 2
[19896461607] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned virtio_netd (PID=17)
[19901544102] [INFO] [sprout::pipelines] [CPU2] SPROUT: spawn_netd start
[19903341744] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/netd
[19904586009] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19907091303] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19928839293] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=21d000 exec=false
[19933589709] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=223000 exec=false
[19954402248] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 18 (user task/process) assigned to CPU 3
[19958097159] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned netd (PID=18)
[19958956512] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb01095c8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19961083857] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=18 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369716624 RFLAGS_BEFORE=134 CR3_BEFORE=69156864 fs_base=0 gs_base=18446744071564586640
[19965104940] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00fab68
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x475
[19966762629] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=17 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369650672 RFLAGS_BEFORE=134 CR3_BEFORE=69021696 fs_base=0 gs_base=18446744071564586608
[19969922874] [INFO] [netd] [CPU3] NETD: Starting network stack service...
[19971527334] [INFO] [netd] [CPU3] NETD: Looking for virtio_netd driver service...
[19972713750] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Starting VirtIO-NET driver service...
[19974664545] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Searching for NIC device...
[19977502545] [INFO] [ps2_kbd] [CPU2] ps2_kbd: entering cooperative polling loop
[19979168550] [INFO] [ps2_kbd] [CPU2] ps2_kbd: using polling mode (10ms interval)
[19984874943] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Found NIC device t475
[19987483890] [INFO] [virtio::device] [CPU2] VirtIO: device::new(0x475) - claiming...
[19988826396] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 17 claimed device 1141 (handle 2)
[19990532694] [INFO] [virtio::device] [CPU2] VirtIO: claimed, handle=2
[20002440876] [INFO] [virtio::device] [CPU2] VirtIO: common_bar=4 common_off=0x0 notify_bar=4 notify_off=0x3000 mult=4
[20008760211] [INFO] [virtio::device] [CPU2] VirtIO: mapping common BAR4...
[20010219537] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR4 phys=0xc000004000 size=0x4000 -> virt=0x107f3000
[20011889568] [INFO] [virtio::device] [CPU2] VirtIO: common_cfg at 0x107f3000
[20012990646] [INFO] [virtio::device] [CPU2] VirtIO: notify_cfg at 0x107f6000
[20014779576] [INFO] [virtio::device] [CPU2] VirtIO: device_cfg = Some(276779008)
[20015746146] [INFO] [virtio::device] [CPU2] VirtIO: allocating DMA command buffer...
[20017537815] [INFO] [virtio::device] [CPU2] VirtIO: cmd_buf virt=0x107f7000 phys=0x4256000
[20018885634] [INFO] [virtio::device] [CPU2] VirtIO: device::new complete
[20021107458] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Device features 0x30bf8024
[20023096104] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: MAC 52:54:00:12:34:56
[20024155470] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Link UP
[20033091111] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX queue 0 setup (size=32)
[20041384869] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX queue 1 setup (size=32)
[20050231905] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Allocated RX pool (16 pages, 32 buffers)
[20052623976] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Allocated TX buffer (1 page)
[20054686443] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX queue filled with 32 buffers
[20056168506] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: DRIVER_OK set, device is live
[20057573052] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Driver initialized successfully
[20058931035] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Driver initialized successfully
[20060194605] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: MAC 52:54:00:12:34:56
[20061094845] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Waiting for link...
[20061956409] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Link is UP
[20067389628] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created port (write=15, read=16)
[20073097143] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created RX port (write=17, read=18)
[20081768619] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created service node ThingId([218, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[20091866322] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 2] Starting Network Apps and Services
[20094173088] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up network apps...
[20095965285] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/anther
[20096691153] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20098434147] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20101689300] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([218, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[20164630431] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=268000 exec=false
[20189314959] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=28e000 exec=false
[20197083687] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 19 (user task/process) assigned to CPU 1
[20200428204] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned anther (PID=19)
[20201401077] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00f1068
[20203716621] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/nectar
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[20205448428] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20206706817] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=19 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369914832 RFLAGS_BEFORE=130 CR3_BEFORE=69668864 fs_base=0 gs_base=18446744071564586576
[20210069979] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20212888179] [INFO] [anther] [CPU1] anther: Starting HTTP server (ThingOS anther v0.1)
[20215491516] [INFO] [anther] [CPU1] anther: Starting server mode on port 80...
[20219564376] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[20220889623] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[20237284287] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=226000 exec=false
[20240344971] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=22a000 exec=false
[20247916623] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 20 (user task/process) assigned to CPU 2
[20250743997] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned nectar (PID=20)
[20252817486] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/fetchd
[20253524907] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20255059473] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20260720062] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[20263079232] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[20270329464] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 21 (user task/process) assigned to CPU 3
[20273619894] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010a670
[20275102353] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned fetchd (PID=21)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[20276977545] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=21 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370047152 RFLAGS_BEFORE=130 CR3_BEFORE=70602752 fs_base=0 gs_base=18446744071564586640
[20282322852] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/clock
[20283059907] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20284673937] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20285402247] [INFO] [fetchd] [CPU3] FETCHD: Starting IP address display...
[20286773859] [INFO] [fetchd] [CPU3] FETCHD: Waiting for UI Root (Compositor)...
[20292343599] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[20296068936] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20e000 exec=false
[20303580858] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 22 (user task/process) assigned to CPU 1
[20306192511] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb010a670
[20308193400] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned clock (PID=22)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[20309902668] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=22 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370113712 RFLAGS_BEFORE=130 CR3_BEFORE=70729728 fs_base=0 gs_base=18446744071564586576
[20314708887] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/taskman
[20315938137] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20318332551] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20327965383] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[20331061971] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=210000 exec=false
[20338189245] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 23 (user task/process) assigned to CPU 2
[20341291179] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned taskman (PID=23)
[20343523101] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/fontd
[20344281276] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20345721198] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20369828655] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=226000 exec=false
[20374777467] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=22d000 exec=false
[20382441750] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 24 (user task/process) assigned to CPU 3
[20385181773] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010bc50
[20386479234] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned fontd (PID=24)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[20388103395] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=24 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370246320 RFLAGS_BEFORE=130 CR3_BEFORE=71032832 fs_base=0 gs_base=18446744071564586640
[20391542655] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/blossom
[20392600899] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20394313236] [INFO] [fontd] [CPU3] FONTD: Starting Graph-Native Font Service v2 (Atlas-based IPC)
[20395368939] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20411846664] [INFO] [fontd] [CPU3] FONTD: Service node created, req=19, resp=22
[20413726377] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=219000 exec=false
[20415899691] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[20417211738] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=21d000 exec=false
[20418170619] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[20419782834] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[20421509625] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=227 pred=0 subj_lo=0
[20425772235] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 25 (user task/process) assigned to CPU 1
[20428575948] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned blossom (PID=25)
[20430477243] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/flytrap
[20431324782] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20433118002] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20434328442] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00f1030
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[20436084009] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=25 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370331328 RFLAGS_BEFORE=134 CR3_BEFORE=71307264 fs_base=0 gs_base=18446744071564586576
[20441513433] [INFO] [blossom] [CPU1] BLOSSOM: Starting SVG Cache Service
[20443849833] [INFO] [blossom] [CPU1] BLOSSOM: Init UI pipeline...
[20447839830] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[20449328922] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[20451277077] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[20452953114] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[20454363369] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[20456320995] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=232 subj_lo=0
[20458272879] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[20459463684] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=231 pred=0 subj_lo=0
[20472533499] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=237000 exec=false
[20483337237] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=246000 exec=false
[20490935454] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 26 (user task/process) assigned to CPU 2
[20493866019] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned flytrap (PID=26)
[20496136848] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[20497286535] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[20498453943] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[20499686526] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=233 pred=0 subj_lo=0
[20502191721] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[20503978902] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[20505843105] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[20507805483] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=234 subj_lo=0
[20509974375] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00f1068
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[20512523493] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=20 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369981072 RFLAGS_BEFORE=134 CR3_BEFORE=70340608 fs_base=0 gs_base=18446744071564586608
[20519210349] [INFO] [nectar] [CPU2] NECTAR: Started.
[20522979180] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00f10e8
[20524300632] [INFO] [fontd] [CPU3] FONTD: Opened ASSET watch (handle=1248) for kind 'Asset'
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[20526106986] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=23 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370180336 RFLAGS_BEFORE=134 CR3_BEFORE=70877184 fs_base=0 gs_base=18446744071564586608
[20530518261] [INFO] [fontd] [CPU3] FONTD: Service ready
[20532353424] [INFO] [taskman] [CPU2] TASKMAN: starting task manager
[20534763051] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[20535949467] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[20537236071] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[20538670251] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=237 subj_lo=0
[20540364933] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00f1030
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[20542758159] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=26 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370398880 RFLAGS_BEFORE=134 CR3_BEFORE=71516160 fs_base=0 gs_base=18446744071564586608
[20550040599] [INFO] [flytrap] [CPU2] FLYTRAP: Starting unified content provider service...
[20552858832] [INFO] [flytrap] [CPU2] FLYTRAP: Service contract validated - graph-native asset watcher
[20553983538] [INFO] [ps2_mouse] [CPU3] ps2_mouse: init done
[20554653867] [INFO] [flytrap] [CPU2] FLYTRAP: Initializing Limine module content source...
[20555586975] [INFO] [kernel::syscall::handlers::device] [CPU3] DEVICE: task subscribed to vector 0x2c
[20557225854] [INFO] [ps2_mouse] [CPU3] ps2_mouse: subscribed to IRQ12 (vector 0x2c)
[20560153812] [INFO] [ps2_mouse] [CPU3] ps2_mouse: entering interrupt-driven loop
[20562282477] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[20563214760] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 3] Starting Compositor
[20564293662] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[20565103680] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([218, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[20577811089] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Published service - TX port=15, RX port=18
[20581216524] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[20582368323] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[20583560481] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[20585187810] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=238 subj_lo=0
[20588930571] [INFO] [sprout::pipelines] [CPU2] SPROUT: Writing bloom BS: drv_req=1, drv_resp=4, bristle_evt=12
[20602068069] [INFO] [sprout::pipelines] [CPU2] SPROUT: Bloom handles via BS=1252 backend=VirtIO-GPU
[20604380346] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/bloom
[20605259070] [INFO] [netd] [CPU3] NETD: Driver TX port=15, RX port=18, link_up=true, mtu=1500
[20606405985] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20608098852] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20609353215] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[20610565503] [INFO] [netd] [CPU3] NETD: Connected to driver - MAC 52:54:00:12:34:56
[20611552236] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[20613051195] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[20614535700] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=245 subj_lo=0
[20616117159] [INFO] [netd] [CPU3] NETD: Created socket API port (write=23, read=24)
[20626756689] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[20627939937] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[20629215882] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[20630663262] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=246 subj_lo=0
[20642885901] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[20644072845] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[20645356842] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[20646814683] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=247 pred=0 subj_lo=0
[20661294027] [INFO] [netd] [CPU3] NETD: Published initial stack node ThingId([233, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) to graph
[20706565440] [INFO] [netd] [CPU3] NETD: Starting DHCP...
[20708109378] [INFO] [netd::dhcp] [CPU3] DHCP: Starting discovery...
[20713609026] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 304 bytes - IPv4 other
[20715950145] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=23, our_write=25, our_read=26)
[20717917176] [INFO] [anther] [CPU1] anther: Connected to network stack
[20744827455] [INFO] [netd::dhcp] [CPU3] DHCP: Deconfigured
[20759005278] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=2d4000 exec=false
[20773283124] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=2ea000 exec=false
[20781115839] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 27 (user task/process) assigned to CPU 3
[20783962617] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db458
[20785093560] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned bloom (PID=27)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x4e4
[20786702244] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=27 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370542976 RFLAGS_BEFORE=130 CR3_BEFORE=72167424 fs_base=0 gs_base=18446744071564586640
[20790132132] [INFO] [blossom] [CPU1] BLOSSOM: UI pipeline ready
[20790806256] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/echo
[20791547634] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20793091275] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20797544295] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=204000 exec=false
[20799040845] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[20801090046] [INFO] [bloom::logging] [CPU3] bloom: logging initialized
[20807129310] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 28 (user task/process) assigned to CPU 1
[20809956090] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned echo (PID=28)
[20812021164] [INFO] [sprout::pipelines] [CPU2] SPROUT: Compositor ready
[20814250347] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db458
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xe
[20815702182] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 304 bytes
[20816442570] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=28 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370624896 RFLAGS_BEFORE=130 CR3_BEFORE=73748480 fs_base=0 gs_base=18446744071564586576
[20829003492] [INFO] [bloom] [CPU3] bloom: [bloom] SIMD backend: SSE2 (x86_64)
[20831885316] [INFO] [blossom] [CPU1] BLOSSOM: Service node created, req=27, resp=30
[20833567293] [INFO] [blossom] [CPU1] BLOSSOM: Service ready
[20841085584] [INFO] [echo] [CPU1] echo: starting up
[20843276487] [INFO] [echo] [CPU1] echo: ready for Bristle events (main loop using handle 14)
[20849850318] [INFO] [bloom] [CPU3] [bloom] EARLY boot args: bristle_evt=12 arg_req=1 arg_resp=4
[20851730856] [INFO] [bloom::compositor] [CPU3] bloom: map_from_bytespace start bs=1233
[20852683731] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=0 len=600
[20866054176] [INFO] [bloom::compositor] [CPU3] bloom: symbols loaded
[20867934879] [INFO] [bloom::compositor] [CPU3] bloom: loop iter 1
[20872734894] [INFO] [bloom::compositor] [CPU3] bloom: got width 1920
[20878975161] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 590 byte frame (594 encoded) to netd rx_port=17
[20881493094] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (594 bytes sent)
[20885789529] [INFO] [bloom::compositor] [CPU3] bloom: got properties w=1920 h=1080 s=7680 f=1
[20887561398] [INFO] [bloom::compositor] [CPU3] bloom: properties OK, building from config...
[20889630993] [INFO] [bloom::compositor] [CPU3] bloom: compositor bytespace 1233 (1920x1080 stride=7680 format=1)
[20900826804] [INFO] [bloom::compositor] [CPU3] bloom: display backend: VirtIO-GPU
[20904777465] [INFO] [bloom::compositor] [CPU3] bloom: mapped size=8294400 (source=bytespace_info)
[20908938864] [INFO] [flytrap] [CPU2] FLYTRAP: Created Limine ContentSource node
[20909983050] [INFO] [flytrap] [CPU2] FLYTRAP: Performing initial boot module scan...
[20930588415] [INFO] [bloom::present] [CPU3] bloom: driver REGISTER (kind=2 caps=0x3)
[20933655204] [INFO] [bloom::present] [CPU3] display: negotiated proto 1.0 caps=DIRTY_RECTS|FULLFRAME max_rects=32
[20935052325] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: received MSG_HELLO
[20953572849] [INFO] [bloom] [CPU3] [bloom] Requesting initial driver buffer...
[20959896177] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 4] Proof of life (Beeper)
[20961773844] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawning beeper...
[20962846509] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 590 bytes
[20963899209] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/beeper
[20964812517] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20966572605] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20971453008] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 316 bytes - IPv4 other
[20972541678] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[20974531248] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[20978168805] [INFO] [bloom] [CPU3] [bloom] ACQUIRED RETURNED!
[20982843189] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 29 (user task/process) assigned to CPU 0
[20986772136] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned beeper (PID=29)
[20989607991] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 5] Starting Discovered User Apps
[20991975840] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/clock'
[20994752889] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/clock
[20996045664] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20998638804] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21009705684] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[21014999808] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20e000 exec=false
[21016362147] [INFO] [bloom] [CPU3] [bloom] ACQUIRED initial driver buffer: 0x11018000 (bs_id=ThingId([214, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]))
[21018562224] [INFO] [bloom] [CPU3] bloom: creating surface...
[21019649640] [INFO] [bloom] [CPU3] bloom: surface created!
[21020559516] [INFO] [bloom] [CPU3] bloom: finding UI CROWN...
[21023296833] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 30 (user task/process) assigned to CPU 3
[21026189415] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=30)
[21027696228] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/hello_std'
[21029334546] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/hello_std
[21030210036] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21032101266] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21035229336] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=202000 exec=false
[21036496371] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=203000 exec=false
[21037649094] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010e310
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[21039872436] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=30 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370760064 RFLAGS_BEFORE=130 CR3_BEFORE=74809344 fs_base=0 gs_base=18446744071564586640
[21044615328] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 31 (user task/process) assigned to CPU 1
[21047167086] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=31)
[21048859128] [INFO] [bloom] [CPU3] bloom: creating new UI CROWN
[21049587834] [INFO] [sprout::supervisor] [CPU2] SPROUT: Startup complete. Entering monitor loop.
[21056827605] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb010e310
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[21058762131] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=31 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370825600 RFLAGS_BEFORE=130 CR3_BEFORE=74969088 fs_base=0 gs_base=18446744071564586576
[21062342631] [INFO] [stem::ui] [CPU3] UiBuilder: created root 1263
[21063295737] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 316 bytes
[21064800669] [ERROR] [kernel::trap] [CPU1] user_invalid_opcode rip=0x0000000000201420 err=0x0000 rsp=0x00000000007fff08 cs=0x2b kind=6
[21069809013] [INFO] [bloom] [CPU3] bloom: UI CROWN initialized!
[21070530129] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=1 len=600
[21072438552] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 590 byte frame (594 encoded) to netd rx_port=17
[21075388785] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (594 bytes sent)
[21081297765] [INFO] [taskman] [CPU2] TASKMAN: Found UI Crown (attempt 3)
[21083349870] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 32 (user thread) assigned to CPU 3
[21086576544] [INFO] [bloom] [CPU3] bloom: spawned asset watcher (tid=32)
[21088544763] [INFO] [bloom::frame_loop] [CPU3] bloom: running (fps_target=60)
[21090077382] [INFO] [bloom] [CPU3] [bloom] bristle_evt_handle = 12 (legacy was 12)
[21095324217] [INFO] [fetchd] [CPU3] FETCHD: Found UI Root: 1263
[21098435556] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from 
```
</details>
