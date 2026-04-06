# ✅ Scenario: User inspects the system graph

> Last run: 2026-04-05 17:45:26

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is running | ✅ | 6478ms | - - - |
| 2 | And I wait for the system to reach ready state | ⏭️ | 0ms | - - - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[13315842012] [INFO] [kernel] [CPU0] BOOTFB: width=1920 height=1080 pitch=7680 bpp=32 format=Bgrx8888
[13322778447] [CONTRACT] [kernel] [CPU0] thing-os kernel starting...
[13327018155] [INFO] [bran::requests] [CPU0] Limine: Found 39 boot modules
[13329530874] [INFO] [bran::requests] [CPU0]   [0] /boot/sprout (cmdline='init') size=111120
[13331243442] [INFO] [bran::requests] [CPU0]   [1] /boot/bristle (cmdline='init') size=41456
[13332314589] [INFO] [bran::requests] [CPU0]   [2] /boot/rtc_cmos (cmdline='init') size=29248
[13333450251] [INFO] [bran::requests] [CPU0]   [3] /boot/clock (cmdline='') size=61968
[13334430516] [INFO] [bran::requests] [CPU0]   [4] /boot/taskman (cmdline='') size=70160
[13335428766] [INFO] [bran::requests] [CPU0]   [5] /boot/ps2_kbd (cmdline='init') size=25072
[13336500606] [INFO] [bran::requests] [CPU0]   [6] /boot/echo (cmdline='init') size=25072
[13337508855] [INFO] [bran::requests] [CPU0]   [7] /boot/bloom (cmdline='init') size=965128
[13338526707] [INFO] [bran::requests] [CPU0]   [8] /boot/ps2_mouse (cmdline='init') size=29168
[13339711308] [INFO] [bran::requests] [CPU0]   [9] /boot/display_bootfb (cmdline='init') size=29248
[13340812188] [INFO] [bran::requests] [CPU0]   [10] /boot/display_virtio_gpu (cmdline='init') size=53864
[13341965109] [INFO] [bran::requests] [CPU0]   [11] /boot/fontd (cmdline='init') size=188944
[13343029194] [INFO] [bran::requests] [CPU0]   [12] /boot/blossom (cmdline='init') size=123408
[13344096447] [INFO] [bran::requests] [CPU0]   [13] /boot/flytrap (cmdline='') size=291352
[13345122483] [INFO] [bran::requests] [CPU0]   [14] /boot/virtio_netd (cmdline='') size=49680
[13346195148] [INFO] [bran::requests] [CPU0]   [15] /boot/rtl8168d (cmdline='') size=45584
[13347211515] [INFO] [bran::requests] [CPU0]   [16] /boot/netd (cmdline='') size=147984
[13348185807] [INFO] [bran::requests] [CPU0]   [17] /boot/fetchd (cmdline='') size=41488
[13349208873] [INFO] [bran::requests] [CPU0]   [18] /boot/anther (cmdline='') size=586272
[13350214416] [INFO] [bran::requests] [CPU0]   [19] /boot/fortune (cmdline='') size=61968
[13351219893] [INFO] [bran::requests] [CPU0]   [20] /boot/photosynthesis (cmdline='') size=12648
[13352315526] [INFO] [bran::requests] [CPU0]   [21] /boot/ahci_disk (cmdline='') size=45664
[13353347436] [INFO] [bran::requests] [CPU0]   [22] /boot/iso9660d (cmdline='') size=53856
[13354353639] [INFO] [bran::requests] [CPU0]   [23] /boot/virtio_sound (cmdline='init') size=49648
[13355459403] [INFO] [bran::requests] [CPU0]   [24] /boot/hdaudio (cmdline='') size=37360
[13356464814] [INFO] [bran::requests] [CPU0]   [25] /boot/pci_stubd (cmdline='') size=29168
[13357505700] [INFO] [bran::requests] [CPU0]   [26] /boot/beeper (cmdline='init') size=33296
[13358546685] [INFO] [bran::requests] [CPU0]   [27] /boot/nectar (cmdline='init') size=176656
[13359615852] [INFO] [bran::requests] [CPU0]   [28] /boot/hello_std (cmdline='') size=16880
[13360706535] [INFO] [bran::requests] [CPU0]   [29] /boot/stdio_demo (cmdline='') size=16880
[13361739666] [INFO] [bran::requests] [CPU0]   [30] /boot/telnetd (cmdline='') size=57872
[13362774513] [INFO] [bran::requests] [CPU0]   [31] /assets/wallpapers/clouds.bmp (cmdline='') size=786486
[13363953339] [INFO] [bran::requests] [CPU0]   [32] /assets/wallpapers/flower.bmp (cmdline='') size=8386614
[13365135069] [INFO] [bran::requests] [CPU0]   [33] /assets/wallpapers/leather.bmp (cmdline='') size=1179702
[13366353033] [INFO] [bran::requests] [CPU0]   [34] /assets/wallpapers/linen.bmp (cmdline='') size=4718646
[13367528196] [INFO] [bran::requests] [CPU0]   [35] /assets/fonts/NotoSans-Regular.ttf (cmdline='') size=569208
[13368767016] [INFO] [bran::requests] [CPU0]   [36] /assets/themes/genie_circles.wasm (cmdline='') size=2824
[13369953927] [INFO] [bran::requests] [CPU0]   [37] /assets/cursors/future/default.svg (cmdline='') size=3051
[13371141564] [INFO] [bran::requests] [CPU0]   [38] /boot/locale.conf (cmdline='') size=85
[13372978212] [INFO] [kernel::memory] [CPU0] Memory map has 64 entries
[13374835650] [INFO] [kernel::memory] [CPU0]   [0] 0x0 - 0x87000 (Usable)
[13376170104] [INFO] [kernel::memory] [CPU0]   [1] 0x87000 - 0x88000 (Reserved)
[13377121197] [INFO] [kernel::memory] [CPU0]   [2] 0x88000 - 0xa0000 (Usable)
[13377980418] [INFO] [kernel::memory] [CPU0]   [3] 0x100000 - 0x800000 (Usable)
[13378876566] [INFO] [kernel::memory] [CPU0]   [4] 0x800000 - 0x808000 (Other)
[13379827065] [INFO] [kernel::memory] [CPU0]   [5] 0x808000 - 0x80b000 (Usable)
[13380701136] [INFO] [kernel::memory] [CPU0]   [6] 0x80b000 - 0x80c000 (Other)
[13381563426] [INFO] [kernel::memory] [CPU0]   [7] 0x80c000 - 0x811000 (Usable)
[13382460267] [INFO] [kernel::memory] [CPU0]   [8] 0x811000 - 0x900000 (Other)
[13383321501] [INFO] [kernel::memory] [CPU0]   [9] 0x900000 - 0x1780000 (Reserved)
[13384232697] [INFO] [kernel::memory] [CPU0]   [10] 0x1780000 - 0x7797d000 (Usable)
[13385181678] [INFO] [kernel::memory] [CPU0]   [11] 0x7797d000 - 0x779e1000 (Reserved)
[13386128481] [INFO] [kernel::memory] [CPU0]   [12] 0x779e1000 - 0x779e2000 (Other)
[13387050600] [INFO] [kernel::memory] [CPU0]   [13] 0x779e2000 - 0x779e3000 (Reserved)
[13388015091] [INFO] [kernel::memory] [CPU0]   [14] 0x779e3000 - 0x779e4000 (Other)
[13389121449] [INFO] [kernel::memory] [CPU0]   [15] 0x779e4000 - 0x779e5000 (Reserved)
[13390058748] [INFO] [kernel::memory] [CPU0]   [16] 0x779e5000 - 0x779e6000 (Other)
[13390963674] [INFO] [kernel::memory] [CPU0]   [17] 0x779e6000 - 0x779e7000 (Reserved)
[13391926482] [INFO] [kernel::memory] [CPU0]   [18] 0x779e7000 - 0x77a72000 (Other)
[13392836457] [INFO] [kernel::memory] [CPU0]   [19] 0x77a72000 - 0x77a73000 (Reserved)
[13393776792] [INFO] [kernel::memory] [CPU0]   [20] 0x77a73000 - 0x77ef4000 (Other)
[13394683401] [INFO] [kernel::memory] [CPU0]   [21] 0x77ef4000 - 0x77ef5000 (Reserved)
[13395659970] [INFO] [kernel::memory] [CPU0]   [22] 0x77ef5000 - 0x78016000 (Other)
[13396568460] [INFO] [kernel::memory] [CPU0]   [23] 0x78016000 - 0x78017000 (Reserved)
[13397512755] [INFO] [kernel::memory] [CPU0]   [24] 0x78017000 - 0x78817000 (Other)
[13398452859] [INFO] [kernel::memory] [CPU0]   [25] 0x78817000 - 0x78818000 (Reserved)
[13399395306] [INFO] [kernel::memory] [CPU0]   [26] 0x78818000 - 0x788d9000 (Other)
[13400311782] [INFO] [kernel::memory] [CPU0]   [27] 0x788d9000 - 0x788da000 (Reserved)
[13401255945] [INFO] [kernel::memory] [CPU0]   [28] 0x788da000 - 0x788e9000 (Other)
[13402196049] [INFO] [kernel::memory] [CPU0]   [29] 0x788e9000 - 0x788ea000 (Reserved)
[13403139618] [INFO] [kernel::memory] [CPU0]   [30] 0x788ea000 - 0x788ef000 (Other)
[13404048570] [INFO] [kernel::memory] [CPU0]   [31] 0x788ef000 - 0x788f0000 (Reserved)
[13405000092] [INFO] [kernel::memory] [CPU0]   [32] 0x788f0000 - 0x788f5000 (Other)
[13405913631] [INFO] [kernel::memory] [CPU0]   [33] 0x788f5000 - 0x788f6000 (Reserved)
[13406857695] [INFO] [kernel::memory] [CPU0]   [34] 0x788f6000 - 0x78922000 (Other)
[13407766416] [INFO] [kernel::memory] [CPU0]   [35] 0x78922000 - 0x78924000 (Reserved)
[13408735296] [INFO] [kernel::memory] [CPU0]   [36] 0x78924000 - 0x7892d000 (Other)
[13409651739] [INFO] [kernel::memory] [CPU0]   [37] 0x7892d000 - 0x7892f000 (Reserved)
[13410599961] [INFO] [kernel::memory] [CPU0]   [38] 0x7892f000 - 0x78937000 (Other)
[13411531518] [INFO] [kernel::memory] [CPU0]   [39] 0x78937000 - 0x78938000 (Reserved)
[13412477892] [INFO] [kernel::memory] [CPU0]   [40] 0x78938000 - 0x78942000 (Other)
[13413387042] [INFO] [kernel::memory] [CPU0]   [41] 0x78942000 - 0x78943000 (Reserved)
[13414325595] [INFO] [kernel::memory] [CPU0]   [42] 0x78943000 - 0x78950000 (Other)
[13415251542] [INFO] [kernel::memory] [CPU0]   [43] 0x78950000 - 0x78952000 (Reserved)
[13416192141] [INFO] [kernel::memory] [CPU0]   [44] 0x78952000 - 0x78960000 (Other)
[13417104129] [INFO] [kernel::memory] [CPU0]   [45] 0x78960000 - 0x78961000 (Reserved)
[13418069346] [INFO] [kernel::memory] [CPU0]   [46] 0x78961000 - 0x7896d000 (Other)
[13418981367] [INFO] [kernel::memory] [CPU0]   [47] 0x7896d000 - 0x7896e000 (Reserved)
[13419917313] [INFO] [kernel::memory] [CPU0]   [48] 0x7896e000 - 0x78972000 (Other)
[13420828146] [INFO] [kernel::memory] [CPU0]   [49] 0x78972000 - 0x78973000 (Reserved)
[13421787720] [INFO] [kernel::memory] [CPU0]   [50] 0x78973000 - 0x78983000 (Other)
[13422700830] [INFO] [kernel::memory] [CPU0]   [51] 0x78983000 - 0x78984000 (Reserved)
[13423641264] [INFO] [kernel::memory] [CPU0]   [52] 0x78984000 - 0x78a14000 (Other)
[13424603973] [INFO] [kernel::memory] [CPU0]   [53] 0x78a14000 - 0x78a15000 (Reserved)
[13425557046] [INFO] [kernel::memory] [CPU0]   [54] 0x78a15000 - 0x78a20000 (Other)
[13426476591] [INFO] [kernel::memory] [CPU0]   [55] 0x78a20000 - 0x78a21000 (Reserved)
[13427417091] [INFO] [kernel::memory] [CPU0]   [56] 0x78a21000 - 0x78a46000 (Other)
[13428354621] [INFO] [kernel::memory] [CPU0]   [57] 0x78a46000 - 0x78a47000 (Reserved)
[13429292877] [INFO] [kernel::memory] [CPU0]   [58] 0x78a47000 - 0x78a53000 (Other)
[13430211102] [INFO] [kernel::memory] [CPU0]   [59] 0x78a53000 - 0x78a54000 (Reserved)
[13431180444] [INFO] [kernel::memory] [CPU0]   [60] 0x78a54000 - 0x78a61000 (Other)
[13432091310] [INFO] [kernel::memory] [CPU0]   [61] 0x78a61000 - 0x78a62000 (Reserved)
[13433042502] [INFO] [kernel::memory] [CPU0]   [62] 0x78a62000 - 0x78aaa000 (Other)
[13433959572] [INFO] [kernel::memory] [CPU0]   [63] 0x78aaa000 - 0x78aab000 (Reserved)
[13435232052] [INFO] [kernel::memory] [CPU0] HHDM Offset: 0xffff800000000000
[13699160178] [CONTRACT] [kernel::memory] [CPU0] Frame allocator initialized with 485781 free frames
[13710794526] [INFO] [bran::arch] [CPU0] IOAPIC: hhdm=0xffff800000000000
[13716884676] [INFO] [bran::arch] [CPU0] IOAPIC: Disabling legacy PIC...
[13718654037] [INFO] [bran::arch] [CPU0] IOAPIC: PIC disabled OK
[13719876060] [INFO] [bran::arch] [CPU0] IOAPIC: RSDP virt=0x7f77e014
[13724807184] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: total length 144
[13727458932] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77802c
[13729169454] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778034
[13730300265] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77803c
[13731449424] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778044
[13732574163] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 1, len 12 at 0xffff80007f77804c
[13734153114] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778058
[13735621548] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778062
[13736768892] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f77806c
[13737929832] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778076
[13739066880] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778080
[13740220032] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 4, len 6 at 0xffff80007f77808a
[13741965237] [INFO] [bran::arch] [CPU0] IOAPIC: MADT parsed OK
[13743384534] [INFO] [bran::arch] [CPU0] SMP: Found 4 CPUs (CPU_COUNT now = 4)
[13744545672] [INFO] [bran::arch] [CPU0] IOAPIC: Found at phys 0xfec00000, GSI base 0
[13746692322] [INFO] [bran::arch] [CPU0] IOAPIC: Registers initialized
[13748052384] [INFO] [bran::arch] [CPU0] IOAPIC: Local APIC enabled (SVR=0x1FF, TPR=0)
[13749532500] [INFO] [bran::arch] [CPU0] IOAPIC: version 0x20, 24 redir entries
[13751905695] [INFO] [bran::arch] [CPU0] IOAPIC: All pins masked
[13754258925] [INFO] [bran::arch] [CPU0] IOAPIC: IRQ1 -> GSI 1 -> 0x21
[13755436101] [INFO] [bran::arch] [CPU0] IOAPIC: IRQ12 -> GSI 12 -> 0x2C
[13756339509] [INFO] [bran::arch] [CPU0] IOAPIC: Init complete
[13757492694] [CONTRACT] [kernel] [CPU0] Initializing global allocator...
[14170019688] [INFO] [kernel::memory::global_alloc] [CPU0] Global allocator initialized (LinkedHeap, 32MB)
[14171474163] [CONTRACT] [kernel] [CPU0] Seeding entropy pool...
[14175207222] [INFO] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (NOT seeded)
[14176625727] [INFO] [kernel::entropy] [CPU0] ENTROPY: marked seeded with weak entropy (timer-only fallback)
[14177906886] [CONTRACT] [kernel] [CPU0] Initializing SIMD...
[14179849035] [CONTRACT] [kernel] [CPU0] Initializing tasking...
[14193519153] [INFO] [kernel::sched] [CPU0]   Acquiring scheduler lock...
[14195358375] [INFO] [kernel::sched] [CPU0]   Lock acquired, checking if initialized...
[14196564162] [INFO] [kernel::sched] [CPU0]   Allocating scheduler...
[14198671674] [INFO] [kernel::sched] [CPU0]   Leaking scheduler...
[14199566634] [INFO] [kernel::sched] [CPU0]   Initializing boot task...
[14202521454] [INFO] [kernel::sched] [CPU0]   Creating boot task...
[14210908833] [INFO] [kernel::sched] [CPU0]   Creating idle tasks...
[14213043900] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 1 assigned to CPU 0
[14227145394] [INFO] [kernel::sched] [CPU0]   Boot task initialized
[14228045832] [INFO] [kernel::sched] [CPU0]   Storing scheduler pointer...
[14246662023] [INFO] [kernel::sched] [CPU0]   Initialized 4 event ring(s)
[14247739506] [CONTRACT] [kernel::sched] [CPU0] Scheduler initialized
[14250346671] [CONTRACT] [alloc] [CPU0] MEDIUM ALLOC #1: 224 KB align=8 total=0MB
[14252056335] [INFO] [kernel::task] [CPU0]   Creating graph worker tasks...
[14253557208] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 2 assigned to CPU 0
[14256384417] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 3 assigned to CPU 0
[14257609014] [INFO] [kernel] [CPU0] System initialized. Setting up preemption timer (100Hz)...
[14293085070] [INFO] [bran::arch::x86_64] [CPU0] LAPIC: calibrated timer (62356700 ticks/sec), init_cnt=623567 for 100Hz
[14295230961] [INFO] [kernel] [CPU0] Kernel: Detected 4 CPUs. Starting 3 secondaries...
[14296472883] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting 3 secondary CPUs...
[14298103611] [INFO] [bran::arch::x86_64] [CPU0] SMP: Initializing trampoline at 0x8000
[14304410307] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 1 (APIC 1)
[14335528944] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 1 (APIC 1) is online
[14336946492] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 2 (APIC 2)
[14338601409] [INFO] [kernel] [CPU1] SMP: Entering kernel_secondary_entry for CPU 1
[14340788946] [INFO] [kernel::sched] [CPU1] SMP: CPU 1 online (triggered by scheduler spawn)
[14342553588] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 4 assigned to CPU 1
[14347391817] [INFO] [kernel::sched] [CPU1] SMP: Secondary CPU 1 online!
[14349607008] [INFO] [kernel::task] [CPU1] SMP: CPU 1 bootstrapped with idle task 4
[14367036222] [INFO] [kernel] [CPU2] SMP: Entering kernel_secondary_entry for CPU 2
[14368428657] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 2 (APIC 2) is online
[14369658237] [INFO] [kernel::sched] [CPU2] SMP: CPU 2 online (triggered by scheduler spawn)
[14370944940] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 3 (APIC 3)
[14372123964] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 5 assigned to CPU 2
[14374113864] [INFO] [kernel::sched] [CPU2] SMP: Secondary CPU 2 online!
[14375168379] [INFO] [kernel::task] [CPU2] SMP: CPU 2 bootstrapped with idle task 5
[14401209438] [INFO] [kernel] [CPU3] SMP: Entering kernel_secondary_entry for CPU 3
[14402544585] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 3 (APIC 3) is online
[14403799080] [INFO] [kernel::sched] [CPU3] SMP: CPU 3 online (triggered by scheduler spawn)
[14405283519] [INFO] [bran::arch::x86_64] [CPU0] SMP: Secondary CPU startup complete
[14406579924] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 6 assigned to CPU 3
[14407804323] [INFO] [kernel] [CPU0] Kernel: Secondary CPU bring-up complete.
[14408902992] [INFO] [kernel::sched] [CPU3] SMP: Secondary CPU 3 online!
[14410038555] [INFO] [kernel::task] [CPU3] SMP: CPU 3 bootstrapped with idle task 6
[14420885787] [INFO] [kernel::root] [CPU0] Spawning Root service...
[14422366662] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 7 assigned to CPU 1
[14424367089] [INFO] [kernel::root::boot_register] [CPU0] ROOT: boot registration begin (Census Phase 1 v0.2)
[14425315410] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 1. Host creation start
[14432183073] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
RM
[14434966755] [INFO] [kernel::root::service] [CPU1] ROOT: started once
[14436144195] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Initializing components...
[14438218311] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Graph initialized
[14439799044] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Journal initialized
[14441188245] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Interner initialized
[14461688175] [CONTRACT] [kernel::root::service] [CPU1] ROOT: LogSymbols initialized
[14466497496] [CONTRACT] [kernel::root::service] [CPU1] ROOT: BatchScratch initialized
[14468437500] [CONTRACT] [kernel::root::service] [CPU1] ROOT: QueryScratch initialized
[14469717042] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Entering main loop
[14533042095] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[14536936161] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[14542694595] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[14545263447] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[14548954035] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[14552864832] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[14556211296] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 1. Host registered: t1
[14557427412] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 2. Platform Bus start
[14559075762] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[14569403013] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[14572125183] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[14576285328] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[14580214638] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[14584085043] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[14588112627] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 2. Platform Bus done
[14589412596] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 3. Kernel start
[14608582560] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 3. Kernel done
[14609707662] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 4. Root Service start
[14621305017] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 4. Root Service done
[14622687519] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 5. CPUs start
[14670508215] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 5. CPUs done
[14671603980] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 6. Memory Ranges start
[15254210763] [CONTRACT] [alloc] [CPU1] MEDIUM ALLOC #2: 120 KB align=8 total=1MB
[16050441693] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 6. Memory Ranges done
[16097408316] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=1000 nodes=331 watches=0 history=498 journal=423 symbols=51 drops=0
[16161200583] [CONTRACT] [alloc] [CPU1] MEDIUM ALLOC #3: 240 KB align=8 total=1MB
[18377283012] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=2000 nodes=447 watches=0 history=962 journal=770 symbols=96 drops=0
[19731883149] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: filesystem bootstrap: /initrd/ with 39 files
[19869851562] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: Census Phase 2: PCI
[19870966467] [INFO] [kernel::root::pci] [CPU0] PCI: Starting enumeration...
[19992465537] [INFO] [kernel::root::pci] [CPU0] PCI: 00:00.0 8086:29c0 Intel Corporation 82G33/G31/P35/P31 Express DRAM Controller class=06:00 prog_if=00 rev=00
[20052009087] [INFO] [kernel::root::pci] [CPU0] PCI: 00:01.0 1af4:1050 Red Hat, Inc. Virtio 1.0 GPU class=03:00 prog_if=00 rev=01
[20082705060] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR0: phys=0x80000000 size=0x800000
[20083595862] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR2: phys=0xc000000000 size=0x4000
[20084248833] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR4: phys=0x80882000 size=0x1000
[20087849925] [INFO] [kernel::root::pci] [CPU0] PCI: Found virtio display controller at 00:01.0
[20106642831] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO notify_cfg BAR2 offset=0x3000 mult=4
[20123612553] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO common_cfg BAR2 offset=0x1000
[20125919781] [INFO] [kernel::root::pci] [CPU0] PCI: Registered virtio GPU (graph_id=1086, idx=1) BAR0=0x80000000
[20181461784] [INFO] [kernel::root::pci] [CPU0] PCI: 00:02.0 1af4:1000 Red Hat, Inc. Virtio network device class=02:00 prog_if=00 rev=00
[20204588844] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR1: phys=0x80881000 size=0x1000
[20205574950] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR4: phys=0xc000004000 size=0x4000
[20212118883] [INFO] [kernel::root::pci] [CPU0] PCI: Found virtio network controller at 00:02.0
[20241369786] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO notify_cfg BAR4 offset=0x3000 mult=4
[20263062963] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO common_cfg BAR4 offset=0x0
[20267773977] [INFO] [kernel::root::pci] [CPU0] PCI: Registered virtio network (graph_id=1120, idx=2) BAR4=0xc000004000
[20268876078] [INFO] [kernel::root::pci] [CPU0] VirtIO-Net: Device registered for userspace driver
[20339223432] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.0 8086:2918 Intel Corporation 82801IB (ICH9) LPC Interface Controller class=06:01 prog_if=00 rev=02
[20341573164] [INFO] [kernel::root::pci] [CPU0] PCI: Found LPC/ISA bridge at 00:1f.0
[20441736975] [INFO] [kernel::root::pci] [CPU0] LPC: Created Legacy IO bus with CMOS and PS/2 controller
[20534804961] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.2 8086:2922 Intel Corporation 82801IR/IO/IH (ICH9R/DO/DH) 6 port SATA Controller [AHCI mode] class=01:06 prog_if=01 rev=02
[20548754061] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR5: phys=0x80880000 size=0x1000
[20553611133] [INFO] [kernel::root::pci] [CPU0] PCI: Found AHCI SATA controller at 00:1f.2
[20556321588] [INFO] [kernel::root::pci] [CPU0] PCI: Registered AHCI controller (graph_id=1193, idx=5) BAR5=0x80880000
[20602615242] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=3000 nodes=584 watches=0 history=1024 journal=1024 symbols=181 drops=0
[20636042889] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.3 8086:2930 Intel Corporation 82801I (ICH9 Family) SMBus Controller class=0c:05 prog_if=00 rev=02
[20641160595] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: registered items. host=t1 kernel=tb
[20642348826] [CONTRACT] [kernel] [CPU0] KERNEL: root census complete: host=t1 kernel=tb root=te
[20643212040] [CONTRACT] [kernel] [CPU0] Kernel: Enumerating 39 boot modules...
[20644238439] [CONTRACT] [kernel] [CPU0]   [0] name='/boot/sprout' cmdline='init' size=111120
[20644948302] [CONTRACT] [kernel] [CPU0]   [1] name='/boot/bristle' cmdline='init' size=41456
[20645753667] [CONTRACT] [kernel] [CPU0]   [2] name='/boot/rtc_cmos' cmdline='init' size=29248
[20646420762] [CONTRACT] [kernel] [CPU0]   [3] name='/boot/clock' cmdline='' size=61968
[20647036410] [CONTRACT] [kernel] [CPU0]   [4] name='/boot/taskman' cmdline='' size=70160
[20647662981] [CONTRACT] [kernel] [CPU0]   [5] name='/boot/ps2_kbd' cmdline='init' size=25072
[20648550879] [CONTRACT] [kernel] [CPU0]   [6] name='/boot/echo' cmdline='init' size=25072
[20649193653] [CONTRACT] [kernel] [CPU0]   [7] name='/boot/bloom' cmdline='init' size=965128
[20649842235] [CONTRACT] [kernel] [CPU0]   [8] name='/boot/ps2_mouse' cmdline='init' size=29168
[20650511805] [CONTRACT] [kernel] [CPU0]   [9] name='/boot/display_bootfb' cmdline='init' size=29248
[20651232393] [CONTRACT] [kernel] [CPU0]   [10] name='/boot/display_virtio_gpu' cmdline='init' size=53864
[20651966676] [CONTRACT] [kernel] [CPU0]   [11] name='/boot/fontd' cmdline='init' size=188944
[20652623277] [CONTRACT] [kernel] [CPU0]   [12] name='/boot/blossom' cmdline='init' size=123408
[20653292946] [CONTRACT] [kernel] [CPU0]   [13] name='/boot/flytrap' cmdline='' size=291352
[20653932387] [CONTRACT] [kernel] [CPU0]   [14] name='/boot/virtio_netd' cmdline='' size=49680
[20654605917] [CONTRACT] [kernel] [CPU0]   [15] name='/boot/rtl8168d' cmdline='' size=45584
[20655247239] [CONTRACT] [kernel] [CPU0]   [16] name='/boot/netd' cmdline='' size=147984
[20655868299] [CONTRACT] [kernel] [CPU0]   [17] name='/boot/fetchd' cmdline='' size=41488
[20656495002] [CONTRACT] [kernel] [CPU0]   [18] name='/boot/anther' cmdline='' size=586272
[20657128965] [CONTRACT] [kernel] [CPU0]   [19] name='/boot/fortune' cmdline='' size=61968
[20657776293] [CONTRACT] [kernel] [CPU0]   [20] name='/boot/photosynthesis' cmdline='' size=12648
[20658459195] [CONTRACT] [kernel] [CPU0]   [21] name='/boot/ahci_disk' cmdline='' size=45664
[20659196184] [CONTRACT] [kernel] [CPU0]   [22] name='/boot/iso9660d' cmdline='' size=53856
[20659898886] [CONTRACT] [kernel] [CPU0]   [23] name='/boot/virtio_sound' cmdline='init' size=49648
[20660601852] [CONTRACT] [kernel] [CPU0]   [24] name='/boot/hdaudio' cmdline='' size=37360
[20661253074] [CONTRACT] [kernel] [CPU0]   [25] name='/boot/pci_stubd' cmdline='' size=29168
[20661901788] [CONTRACT] [kernel] [CPU0]   [26] name='/boot/beeper' cmdline='init' size=33296
[20662557333] [CONTRACT] [kernel] [CPU0]   [27] name='/boot/nectar' cmdline='init' size=176656
[20663217861] [CONTRACT] [kernel] [CPU0]   [28] name='/boot/hello_std' cmdline='' size=16880
[20663864034] [CONTRACT] [kernel] [CPU0]   [29] name='/boot/stdio_demo' cmdline='' size=16880
[20664531162] [CONTRACT] [kernel] [CPU0]   [30] name='/boot/telnetd' cmdline='' size=57872
[20665166016] [CONTRACT] [kernel] [CPU0]   [31] name='/assets/wallpapers/clouds.bmp' cmdline='' size=786486
[20665913136] [CONTRACT] [kernel] [CPU0]   [32] name='/assets/wallpapers/flower.bmp' cmdline='' size=8386614
[20666667780] [CONTRACT] [kernel] [CPU0]   [33] name='/assets/wallpapers/leather.bmp' cmdline='' size=1179702
[20667429684] [CONTRACT] [kernel] [CPU0]   [34] name='/assets/wallpapers/linen.bmp' cmdline='' size=4718646
[20668862049] [CONTRACT] [kernel] [CPU0]   [35] name='/assets/fonts/NotoSans-Regular.ttf' cmdline='' size=569208
[20670899535] [CONTRACT] [kernel] [CPU0]   [36] name='/assets/themes/genie_circles.wasm' cmdline='' size=2824
[20672762913] [CONTRACT] [kernel] [CPU0]   [37] name='/assets/cursors/future/default.svg' cmdline='' size=3051
[20673968799] [CONTRACT] [kernel] [CPU0]   [38] name='/boot/locale.conf' cmdline='' size=85
[20675840658] [INFO] [kernel] [CPU0] Found init module: /boot/sprout (cmdline: 'init'), loading...
[20677892895] [INFO] [kernel::task::loader] [CPU0] Loading module: /boot/sprout
[20679079047] [DEBUG] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20687438970] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[20702485452] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=213000 exec=false
[20707409844] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=21a000 exec=false
[20717525367] [INFO] [kernel] [CPU0] Spawning sprout with registry at 0x600000...
[20718261432] [CONTRACT] [kernel] [CPU0] Spawning init process...
[20720368581] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 8 (user task/process) assigned to CPU 2
[20722933044] [CONTRACT] [kernel] [CPU0] Entering scheduler loop.
[20723668383] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600000
ThingOS Petals
type 'help' for commands

petals> [20731281813] [DEBUG] [kernel::sched::sleep] [CPU0] DIAG ctx_switch: cpu=0 from=0 to=2
[20732517696] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=8 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369013568 RFLAGS_BEFORE=130 CR3_BEFORE=50393088 fs_base=0 gs_base=18446744071564586608
[20757604989] [INFO] [sprout] [CPU2] [sprout] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ff990 rip=0x20139f rflags=0x206
[20765872677] [INFO] [sprout] [CPU2] SPROUT: v0.4 starting (Supervisor Mode)...
[20767759419] [INFO] [sprout::devtree] [CPU2] SPROUT: devtree::init entry (v0.2)
[20769011208] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 1: Find Host
[20779048356] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 2: HHDM
[20785439433] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 3: Platform Bus
[20790088572] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 4: Firmware
[20791788336] [INFO] [sprout::devtree] [CPU2] SPROUT: Finding ACPI...
[20797859874] [INFO] [sprout::devtree] [CPU2] SPROUT: Found 1 ACPI nodes
[20802661110] [INFO] [sprout::devtree] [CPU2] SPROUT: ACPI RSDP = 0x7f77e014
[20804
```
</details>
