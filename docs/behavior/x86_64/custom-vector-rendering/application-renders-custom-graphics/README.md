# ❌ Scenario: Application renders custom graphics

> Last run: 2026-04-05 17:45:26

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is running | ❌ | 121091ms | - - - |

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
[20804401893] [INFO] [sprout::devtree] [CPU2] SPROUT: Finding DTB...
[20811803859] [INFO] [sprout::devtree] [CPU2] SPROUT: Found 0 DTB nodes
[20813620905] [INFO] [sprout::devtree] [CPU2] SPROUT: Init OK, returning context
[20815332549] [INFO] [sprout::devtree] [CPU2] SPROUT: build() called
[20817071121] [INFO] [sprout::devtree::x86_64] [CPU2] SPROUT: x86_64 platform enrichment... (v0.2)
[20821678614] [INFO] [sprout::devtree::x86_64] [CPU2] SPROUT: x86_64 enumerate done
[20823580173] [INFO] [sprout] [CPU2] SPROUT: About to create Supervisor...
[20825582217] [INFO] [sprout] [CPU2] SPROUT: Supervisor created, calling run_forever...
[20827871823] [INFO] [sprout::supervisor] [CPU2] SPROUT: Supervisor starting (phased mode)...
[20830194396] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 1] Hardware Discovery and Core Drivers
[20832693651] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovering modules...
[20839265238] [INFO] [sprout::supervisor] [CPU2] SPROUT: Found 39 modules
[20892170343] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[0] = '/boot/sprout'
[20900886336] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[1] = '/boot/bristle'
[20907527157] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[2] = '/boot/rtc_cmos'
[20914421946] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[3] = '/boot/clock'
[20919081546] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/clock
[20924202486] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[4] = '/boot/taskman'
[20931010188] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[5] = '/boot/ps2_kbd'
[20938024602] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[6] = '/boot/echo'
[20945228073] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[7] = '/boot/bloom'
[20952443292] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[8] = '/boot/ps2_mouse'
[20960248650] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[9] = '/boot/display_bootfb'
[20967985896] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[10] = '/boot/display_virtio_gpu'
[20976792903] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[11] = '/boot/fontd'
[20983864275] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[12] = '/boot/blossom'
[20990685804] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[13] = '/boot/flytrap'
[20998476675] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[14] = '/boot/virtio_netd'
[21006364830] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[15] = '/boot/rtl8168d'
[21013867248] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[16] = '/boot/netd'
[21020662047] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[17] = '/boot/fetchd'
[21027437706] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[18] = '/boot/anther'
[21033829674] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[19] = '/boot/fortune'
[21041013708] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[20] = '/boot/photosynthesis'
[21047842860] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[21] = '/boot/ahci_disk'
[21055105368] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[22] = '/boot/iso9660d'
[21063228120] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[23] = '/boot/virtio_sound'
[21071452644] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[24] = '/boot/hdaudio'
[21078085017] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[25] = '/boot/pci_stubd'
[21084512097] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[26] = '/boot/beeper'
[21090944985] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[27] = '/boot/nectar'
[21098221353] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[28] = '/boot/hello_std'
[21102579036] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/hello_std
[21107651334] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[29] = '/boot/stdio_demo'
[21114417390] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[30] = '/boot/telnetd'
[21120687522] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[31] = '/assets/wallpapers/clouds.bmp'
[21126959997] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[32] = '/assets/wallpapers/flower.bmp'
[21133086876] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[33] = '/assets/wallpapers/leather.bmp'
[21139818315] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[34] = '/assets/wallpapers/linen.bmp'
[21146163192] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[35] = '/assets/fonts/NotoSans-Regular.ttf'
[21152491140] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[36] = '/assets/themes/genie_circles.wasm'
[21158797506] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[37] = '/assets/cursors/future/default.svg'
[21166896597] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[38] = '/boot/locale.conf'
[21171734100] [INFO] [sprout::registry] [CPU2] SPROUT: Scanning boot modules...
[21196509279] [INFO] [sprout::registry] [CPU2] SPROUT: Registering driver 'dev.rtc.Cmos' -> '/boot/rtc_cmos' (fallback)
[21396959694] [INFO] [sprout::registry] [CPU2] SPROUT: Registry scan complete. Found 1 drivers.
[21407540682] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/pci_stubd
[21409128543] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21411989247] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21418664751] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[21420488331] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[21431191947] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 9 (user task/process) assigned to CPU 3
[21438098847] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db400
[21439786467] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned pci_stubd (PID=9)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[21449305185] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=9 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369079104 RFLAGS_BEFORE=130 CR3_BEFORE=50860032 fs_base=0 gs_base=18446744071564586640
[21454425894] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/rtc_cmos
[21455729262] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[21458710779] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21459996987] [INFO] [pci_stubd] [CPU3] pci_stubd: starting pci-id matcher
[21465642990] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=204000 exec=false
[21468866958] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[21477515367] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 10 (user task/process) assigned to CPU 1
[21481553379] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db400
[21483004554] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned rtc_cmos (PID=10)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x49b
[21485171136] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=10 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369144640 RFLAGS_BEFORE=130 CR3_BEFORE=50974720 fs_base=0 gs_base=18446744071564586576
[21489501528] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up storage pipeline...
[21494761728] [INFO] [rtc_cmos] [CPU1] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ffeb0 rip=0x201027 rflags=0x202
[21497378298] [INFO] [rtc_cmos] [CPU1] Starting... arg=49b
[21500016714] [INFO] [rtc_cmos] [CPU1] Serving device ID: ThingId([155, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[21504864183] [INFO] [rtc_cmos] [CPU1] RTC: 2026-04-06 00:47:51 = 1775436471 unix_secs
[21507027828] [INFO] [kernel::time] [CPU1] System clock anchored: unix_secs=1775436471, mono_ns=10753234981, offset=1775436460246765019ns
[21509013372] [INFO] [rtc_cmos] [CPU1] System clock anchored
[21523581618] [INFO] [rtc_cmos] [CPU1] RTC: Set sys.TimeState = 1 (Anchored)
[21578548563] [INFO] [rtc_cmos] [CPU1] Publishing time. Entering maintenance loop.
[21594729123] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ahci_disk
[21596043183] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[21598920222] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21608064093] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[21612110421] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20a000 exec=false
[21624669792] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 11 (user task/process) assigned to CPU 2
[21630439248] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ahci_disk (PID=11)
[21635391822] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[21637836495] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=11 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369210688 RFLAGS_BEFORE=130 CR3_BEFORE=51089408 fs_base=0 gs_base=18446744071564586608
[21645481704] [INFO] [ahci_disk] [CPU2] AHCI: Starting AHCI/SATA disk driver v1
[21652353690] [INFO] [sprout::pipelines] [CPU2] SPROUT: Failed to spawn ata_disk: ENOENT
[21655091766] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/iso9660d
[21656244753] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[21658796643] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21669780858] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[21673366836] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[21684542946] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 12 (user task/process) assigned to CPU 3
[21689750445] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db400
[21691284648] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned iso9660d (PID=12)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[21693298671] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=12 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369277440 RFLAGS_BEFORE=134 CR3_BEFORE=51228672 fs_base=0 gs_base=18446744071564586640
[21698187522] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up audio driver...
[21702834054] [INFO] [iso9660d] [CPU3] ISO9660D: Starting ISO9660 mount service...
[21704837319] [INFO] [ahci_disk] [CPU2] AHCI: Found 6 PCI functions
[21712941591] [INFO] [iso9660d] [CPU3] ISO9660D: Found 0 block devices
[21715087746] [INFO] [iso9660d] [CPU3] ISO9660D: No ISO9660 filesystems found
[21717564495] [INFO] [iso9660d] [CPU3] ISO9660D: Entering service loop with 0 mounts
[21718897332] [INFO] [sprout::pipelines] [CPU2] SPROUT: No Sound device found
[21721211424] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up display pipeline...
[21744282417] [INFO] [sprout::pipelines] [CPU2] SPROUT: Using VirtIO GPU at 1920x1080
[21746712603] [INFO] [sprout::pipelines] [CPU2] SPROUT: Display backend: VirtIO-GPU (1920x1080 stride=7680)
[22374236049] [INFO] [ahci_disk] [CPU2] AHCI: Found AHCI controller at PCI func ThingId([169, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[22379823477] [INFO] [ahci_disk] [CPU2] AHCI: BAR5=0x80880000
[22382978574] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 11 claimed device 1193 (handle 0)
[22384983225] [INFO] [ahci_disk] [CPU2] AHCI: Claimed PCI device ThingId([169, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) handle=0
[22389851451] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR5 phys=0x80880000 size=0x1000 -> virt=0x10000000
[22391495742] [INFO] [ahci_disk] [CPU2] AHCI: Mapped ABAR at 0x10000000
[22393570518] [INFO] [ahci_disk] [CPU2] AHCI: Version 1.0, 6 ports, 32 slots, 64-bit: true
[22394983743] [INFO] [ahci_disk] [CPU2] AHCI: Ports implemented: 0x3f
[22395994533] [INFO] [ahci_disk] [CPU2] AHCI: DMA virt=0x20b000
[22398546060] [INFO] [ahci_disk] [CPU2] AHCI: DMA phys=0x30c8000
[22399531143] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 0...
[22400467551] [INFO] [ahci_disk] [CPU2] AHCI: Port 0 - no device
[22401309711] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 1...
[22402106232] [INFO] [ahci_disk] [CPU2] AHCI: Port 1 - no device
[22402902192] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 2...
[22403984988] [INFO] [ahci_disk] [CPU2] AHCI: Port 2 - SATAPI drive (sig=0xeb140101)
[22408278552] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/display_virtio_gpu
[22409294556] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[22411313991] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[22419060741] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[22421933160] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[22429954701] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 13 (user task/process) assigned to CPU 1
[22433445804] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db400
[22434836622] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned display driver '/display_virtio_gpu' (PID=13)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x30002
[22437226911] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=13 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369352752 RFLAGS_BEFORE=134 CR3_BEFORE=59666432 fs_base=0 gs_base=18446744071564586576
[22471165200] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: starting (drv_req_r=2, drv_resp_w=3)
[22486931346] [INFO] [kernel::syscall::handlers::device] [CPU1] DEVICE: task 13 claimed device 1108 (handle 1)
[22502266446] [INFO] [ahci_disk] [CPU2] AHCI: Registered ATAPI block device 1235 port=2 model='                                        ' rpc_port=5
[22523435715] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up input pipeline (keyboard + mouse)...
[22528271337] [INFO] [sprout::pipelines] [CPU2] SPROUT: Created kbd_raw port (w=7, r=8)
[22532975949] [INFO] [sprout::pipelines] [CPU2] SPROUT: Created mouse_raw port (w=9, r=10)
[22535969610] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ps2_kbd
[22537117449] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[22539836319] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[22545237561] [INFO] [virtio_gpu] [CPU1] virtio_gpu: caps from graph - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[22547191953] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=203000 exec=false
[22548477336] [INFO] [kernel::syscall::handlers::device] [CPU1] DEVICE: Mapped BAR2 phys=0xc000000000 size=0x4000 -> virt=0x10001000
[22550504889] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[22557325791] [INFO] [virtio_gpu] [CPU1] virtio_gpu: device features=0x30000002 virgl=false
[22572117183] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: GPU initialized successfully
[22573532784] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 14 (user task/process) assigned to CPU 2
[22575283104] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: Virgl 3D not supported, using 2D only
[22579193406] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ps2_kbd (PID=14)
[22582346985] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ps2_mouse
[22583610357] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[22586452218] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[22594456434] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[22596330042] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[22601910672] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: creating frame pool 1x 1920x1080 stride=7680 format=2
[22607438106] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 15 (user task/process) assigned to CPU 3
[22611953958] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb0106680
[22613486049] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ps2_mouse (PID=15)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x9
[22615706916] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=15 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369503136 RFLAGS_BEFORE=134 CR3_BEFORE=59940864 fs_base=0 gs_base=18446744071564586640
[22620368760] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/bristle
[22621301538] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[22622994108] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[22624636386] [INFO] [ps2_mouse] [CPU3] ps2_mouse: online (handle=9)
[22626359184] [INFO] [ps2_mouse] [CPU3] ps2_mouse: enabling aux port
[22628803758] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[22631591037] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[22633409898] [INFO] [ps2_mouse] [CPU3] ps2_mouse: controller cfg already correct (0x47)
[22635673302] [INFO] [ps2_mouse] [CPU3] ps2_mouse: sending RESET (0xFF)
[22637983038] [INFO] [ps2_mouse] [CPU3] ps2_mouse: reset ACK received (0xfa)
[22639595121] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 16 (user task/process) assigned to CPU 1
[22640846613] [INFO] [ps2_mouse] [CPU3] ps2_mouse: BAT passed (0xaa), ID 0x00 confirmed
[22642625214] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned bristle (PID=16)
[22643606337] [INFO] [ps2_mouse] [CPU3] ps2_mouse: setting sample rate (100)
[22644759951] [INFO] [sprout::pipelines] [CPU2] SPROUT: Input broker ready (keyboard + mouse)
[22646549277] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up network stack...
[22647594948] [INFO] [ps2_mouse] [CPU3] ps2_mouse: setting resolution (3)
[22650703350] [INFO] [ps2_mouse] [CPU3] ps2_mouse: status result = Some(0) Some(3) Some(100)
[22652830662] [INFO] [ps2_mouse] [CPU3] ps2_mouse: sending enable command (0xF4)
[22654941111] [INFO] [ps2_mouse] [CPU3] ps2_mouse: enable ACK received (0xfa)
[22675307754] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 3...
[22676347716] [INFO] [ahci_disk] [CPU2] AHCI: Port 3 - no device
[22677278580] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 4...
[22678228386] [INFO] [ahci_disk] [CPU2] AHCI: Port 4 - no device
[22679418663] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 5...
[22680713253] [INFO] [ahci_disk] [CPU2] AHCI: Port 5 - no device
[22682293095] [INFO] [ahci_disk] [CPU2] AHCI: Found 1 SATA disk(s)
[22684091562] [INFO] [ahci_disk] [CPU2] AHCI: Entering RPC service loop
[22693962060] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb0105668
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x7
[22696247079] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=14 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369437600 RFLAGS_BEFORE=134 CR3_BEFORE=59805696 fs_base=0 gs_base=18446744071564586608
[22704600468] [INFO] [ps2_kbd] [CPU2] ps2_kbd: online (handle=7)
[23167383327] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb0106680
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x8000a000b000d
[23170000821] [INFO] [ps2_kbd] [CPU2] ps2_kbd: created driver node 1240
[23171245944] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=16 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369585056 RFLAGS_BEFORE=134 CR3_BEFORE=68349952 fs_base=0 gs_base=18446744071564586576
[23175854691] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task subscribed to vector 0x21
[23178825054] [INFO] [ps2_kbd] [CPU2] ps2_kbd: subscribed to IRQ1 (vector 0x21)
[23180678235] [INFO] [bristle] [CPU1] BRISTLE_MAIN_ENTERED_WITH_LOGS_YAY
[23184697635] [INFO] [bristle] [CPU1] bristle: online (kbd=8, mouse=10, evt=11, echo=13)
[23185994535] [INFO] [sprout::pipelines] [CPU2] SPROUT: Found NIC device ThingId([117, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[23189357103] [INFO] [bristle] [CPU1] bristle: created broadcast topic 0
[23190466926] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/virtio_netd
[23191720035] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[23194343568] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[23201828958] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=208000 exec=false
[23204649336] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20b000 exec=false
[23210940258] [INFO] [bristle] [CPU1] bristle: registered in graph as svc.Input (id=1241)
[23213713017] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 17 (user task/process) assigned to CPU 2
[23217279129] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned virtio_netd (PID=17)
[23219275794] [INFO] [sprout::pipelines] [CPU2] SPROUT: spawn_netd start
[23221172865] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/netd
[23223577443] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[23225467650] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[23250396180] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=21d000 exec=false
[23257074093] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=223000 exec=false
[23287739508] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 18 (user task/process) assigned to CPU 3
[23292018849] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned netd (PID=18)
[23292847380] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010a1b8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[23294934069] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=18 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369717120 RFLAGS_BEFORE=130 CR3_BEFORE=68878336 fs_base=0 gs_base=18446744071564586640
[23302134009] [INFO] [netd] [CPU3] NETD: Starting network stack service...
[23303618052] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb0105668
[23305115691] [INFO] [netd] [CPU3] NETD: Looking for virtio_netd driver service...
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x475
[23306997615] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=17 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369650592 RFLAGS_BEFORE=134 CR3_BEFORE=68743168 fs_base=0 gs_base=18446744071564586608
[23315219697] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Starting VirtIO-NET driver service...
[23317792509] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Searching for NIC device...
[23334976038] [INFO] [ps2_kbd] [CPU2] ps2_kbd: entering cooperative polling loop
[23338034412] [INFO] [ps2_kbd] [CPU2] ps2_kbd: using polling mode (10ms interval)
[23343095886] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: frame pool ready (1 buffer)
[23345130831] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Found NIC device t475
[23347482180] [INFO] [virtio::device] [CPU2] VirtIO: device::new(0x475) - claiming...
[23349196596] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 17 claimed device 1141 (handle 2)
[23351613912] [INFO] [virtio::device] [CPU2] VirtIO: claimed, handle=2
[23376538185] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 2] Starting Network Apps and Services
[23378765124] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up network apps...
[23381174289] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/anther
[23382364236] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[23385223653] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[23488500651] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=268000 exec=false
[23526117054] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=28e000 exec=false
[23537840931] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 19 (user task/process) assigned to CPU 1
[23542548546] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb0105668
[23544089184] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned anther (PID=19)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[23546244513] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=19 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369783136 RFLAGS_BEFORE=134 CR3_BEFORE=69554176 fs_base=0 gs_base=18446744071564586576
[23550917280] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/nectar
[23552116269] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[23554586319] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[23555857083] [INFO] [anther] [CPU1] anther: Starting HTTP server (ThingOS anther v0.1)
[23559989046] [INFO] [anther] [CPU1] anther: Starting server mode on port 80...
[23566714050] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[23568964452] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[23588672679] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=226000 exec=false
[23593926972] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=22a000 exec=false
[23614168479] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 20 (user task/process) assigned to CPU 2
[23619423960] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned nectar (PID=20)
[23622577836] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/fetchd
[23623638621] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[23625797349] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[23632801533] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[23636160933] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[23647893819] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 21 (user task/process) assigned to CPU 3
[23651777358] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010b5a8
[23653197117] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned fetchd (PID=21)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[23654933940] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=21 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369915520 RFLAGS_BEFORE=130 CR3_BEFORE=70488064 fs_base=0 gs_base=18446744071564586640
[23659747518] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/clock
[23660770749] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[23662875918] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[23664210537] [INFO] [fetchd] [CPU3] FETCHD: Starting IP address display...
[23666427246] [INFO] [fetchd] [CPU3] FETCHD: Waiting for UI Root (Compositor)...
[23671694937] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[23675868744] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20e000 exec=false
[23683860552] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 22 (user task/process) assigned to CPU 1
[23686970736] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb010b5a8
[23688248232] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned clock (PID=22)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[23689764087] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=22 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369981920 RFLAGS_BEFORE=130 CR3_BEFORE=70615040 fs_base=0 gs_base=18446744071564586576
[23693248491] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/taskman
[23694096261] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[23696072103] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[23706364110] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[23710703610] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=210000 exec=false
[23718620805] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 23 (user task/process) assigned to CPU 2
[23721677991] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned taskman (PID=23)
[23723926677] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/fontd
[23724762798] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[23726365608] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[23752937967] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=226000 exec=false
[23758298982] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=22d000 exec=false
[23766012468] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 24 (user task/process) assigned to CPU 3
[23769290556] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010cc18
[23771042328] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned fontd (PID=24)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[23773064964] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=24 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370114080 RFLAGS_BEFORE=130 CR3_BEFORE=70918144 fs_base=0 gs_base=18446744071564586640
[23777606589] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/blossom
[23778778749] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[23780792376] [INFO] [fontd] [CPU3] FONTD: Starting Graph-Native Font Service v2 (Atlas-based IPC)
[23781953448] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[23804731170] [INFO] [fontd] [CPU3] FONTD: Service node created, req=15, resp=18
[23808744465] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=219000 exec=false
[23809983318] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[23812099575] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[23813970906] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=21d000 exec=false
[23815331265] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[23818093728] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=225 pred=0 subj_lo=0
[23826486717] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 25 (user task/process) assigned to CPU 1
[23832084705] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned blossom (PID=25)
[23834581056] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00f1068
[23836092819] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/flytrap
[23837346159] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[23840075094] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[23841302067] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=25 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370199504 RFLAGS_BEFORE=130 CR3_BEFORE=71192576 fs_base=0 gs_base=18446744071564586576
[23849646150] [INFO] [ps2_mouse] [CPU3] ps2_mouse: init done
[23850883980] [INFO] [kernel::syscall::handlers::device] [CPU3] DEVICE: task subscribed to vector 0x2c
[23852904207] [INFO] [ps2_mouse] [CPU3] ps2_mouse: subscribed to IRQ12 (vector 0x2c)
[23854926480] [INFO] [ps2_mouse] [CPU3] ps2_mouse: entering interrupt-driven loop
[23878476765] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[23880217251] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[23881912428] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[23883639153] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=229 pred=0 subj_lo=0
[23895715140] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=237000 exec=false
[23910961173] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=246000 exec=false
[23912473398] [INFO] [blossom] [CPU1] BLOSSOM: Starting SVG Cache Service
[23914947177] [INFO] [blossom] [CPU1] BLOSSOM: Init UI pipeline...
[23919799134] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[23920963572] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[23922573312] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[23923800912] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[23925257367] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[23926683792] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[23928539316] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=231 subj_lo=0
[23930134470] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=230 pred=0 subj_lo=0
[23932561950] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 26 (user task/process) assigned to CPU 2
[23937866535] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned flytrap (PID=26)
[23943156105] [INFO] [virtio::device] [CPU2] VirtIO: common_bar=4 common_off=0x0 notify_bar=4 notify_off=0x3000 mult=4
[23946802308] [INFO] [fontd] [CPU3] FONTD: Opened ASSET watch (handle=1247) for kind 'Asset'
[23948968659] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb0105668
[23950453197] [INFO] [fontd] [CPU3] FONTD: Service ready
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[23952207279] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=20 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369849440 RFLAGS_BEFORE=134 CR3_BEFORE=70225920 fs_base=0 gs_base=18446744071564586608
[23959032834] [INFO] [nectar] [CPU2] NECTAR: Started.
[23962102890] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb010b5a8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[23964827205] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=23 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370047840 RFLAGS_BEFORE=134 CR3_BEFORE=70762496 fs_base=0 gs_base=18446744071564586608
[23972122152] [INFO] [taskman] [CPU2] TASKMAN: starting task manager
[23975098719] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb010df80
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[23977312557] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=26 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370266832 RFLAGS_BEFORE=130 CR3_BEFORE=71401472 fs_base=0 gs_base=18446744071564586608
[23984901105] [INFO] [flytrap] [CPU2] FLYTRAP: Starting unified content provider service...
[23986944861] [INFO] [flytrap] [CPU2] FLYTRAP: Service contract validated - graph-native asset watcher
[23988944232] [INFO] [flytrap] [CPU2] FLYTRAP: Initializing Limine module content source...
[24007494225] [INFO] [virtio::device] [CPU2] VirtIO: mapping common BAR4...
[24009667209] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR4 phys=0xc000004000 size=0x4000 -> virt=0x107f3000
[24012208836] [INFO] [virtio::device] [CPU2] VirtIO: common_cfg at 0x107f3000
[24014601600] [INFO] [virtio::device] [CPU2] VirtIO: notify_cfg at 0x107f6000
[24016971990] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[24018672876] [INFO] [virtio::device] [CPU2] VirtIO: device_cfg = Some(276779008)
[24019878993] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[24021606015] [INFO] [virtio::device] [CPU2] VirtIO: allocating DMA command buffer...
[24022861533] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[24024746526] [INFO] [virtio::device] [CPU2] VirtIO: cmd_buf virt=0x107f7000 phys=0x44b6000
[24025980990] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=233 subj_lo=0
[24028065633] [INFO] [virtio::device] [CPU2] VirtIO: device::new complete
[24032782818] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 3] Starting Compositor
[24040704072] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Device features 0x30bf8024
[24043367238] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: MAC 52:54:00:12:34:56
[24044824254] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Link UP
[2405582
```
</details>
