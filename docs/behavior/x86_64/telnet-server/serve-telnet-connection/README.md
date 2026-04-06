# ✅ Scenario: Serve telnet connection

> Last run: 2026-04-05 19:25:00

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the telnet server is ready | ✅ | 9215ms | - - - |
| 2 | When I connect to the telnet server and send "match (n) return n;" | ✅ | 16845ms | - [📜](./02/serial.log) - |
| 3 | Then the telnet response should contain "node(" | ✅ | 0ms | - [📜](./03/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[13373272605] [INFO] [kernel] [CPU0] BOOTFB: width=1920 height=1080 pitch=7680 bpp=32 format=Bgrx8888
[13380086247] [CONTRACT] [kernel] [CPU0] thing-os kernel starting...
[13384268337] [INFO] [bran::requests] [CPU0] Limine: Found 39 boot modules
[13386852204] [INFO] [bran::requests] [CPU0]   [0] /boot/sprout (cmdline='init') size=111120
[13388775972] [INFO] [bran::requests] [CPU0]   [1] /boot/bristle (cmdline='init') size=41456
[13389813756] [INFO] [bran::requests] [CPU0]   [2] /boot/rtc_cmos (cmdline='init') size=29248
[13390905561] [INFO] [bran::requests] [CPU0]   [3] /boot/clock (cmdline='') size=61968
[13391850120] [INFO] [bran::requests] [CPU0]   [4] /boot/taskman (cmdline='') size=70160
[13392700992] [INFO] [bran::requests] [CPU0]   [5] /boot/ps2_kbd (cmdline='init') size=25072
[13393329147] [INFO] [bran::requests] [CPU0]   [6] /boot/echo (cmdline='init') size=33264
[13393934763] [INFO] [bran::requests] [CPU0]   [7] /boot/bloom (cmdline='init') size=969224
[13394549883] [INFO] [bran::requests] [CPU0]   [8] /boot/ps2_mouse (cmdline='init') size=29168
[13395289908] [INFO] [bran::requests] [CPU0]   [9] /boot/display_bootfb (cmdline='init') size=29248
[13396018086] [INFO] [bran::requests] [CPU0]   [10] /boot/display_virtio_gpu (cmdline='init') size=53864
[13396720458] [INFO] [bran::requests] [CPU0]   [11] /boot/fontd (cmdline='init') size=188944
[13397413425] [INFO] [bran::requests] [CPU0]   [12] /boot/blossom (cmdline='init') size=123408
[13398050688] [INFO] [bran::requests] [CPU0]   [13] /boot/flytrap (cmdline='') size=291352
[13398674025] [INFO] [bran::requests] [CPU0]   [14] /boot/virtio_netd (cmdline='') size=49680
[13399328514] [INFO] [bran::requests] [CPU0]   [15] /boot/rtl8168d (cmdline='') size=45584
[13399938090] [INFO] [bran::requests] [CPU0]   [16] /boot/netd (cmdline='') size=147984
[13400651913] [INFO] [bran::requests] [CPU0]   [17] /boot/fetchd (cmdline='') size=41488
[13401255780] [INFO] [bran::requests] [CPU0]   [18] /boot/anther (cmdline='') size=586272
[13401934656] [INFO] [bran::requests] [CPU0]   [19] /boot/fortune (cmdline='') size=61968
[13402756125] [INFO] [bran::requests] [CPU0]   [20] /boot/photosynthesis (cmdline='') size=12648
[13403808033] [INFO] [bran::requests] [CPU0]   [21] /boot/ahci_disk (cmdline='') size=45664
[13404808230] [INFO] [bran::requests] [CPU0]   [22] /boot/iso9660d (cmdline='') size=53856
[13405826445] [INFO] [bran::requests] [CPU0]   [23] /boot/virtio_sound (cmdline='init') size=49648
[13406831196] [INFO] [bran::requests] [CPU0]   [24] /boot/hdaudio (cmdline='') size=37360
[13407472749] [INFO] [bran::requests] [CPU0]   [25] /boot/pci_stubd (cmdline='') size=29168
[13408198881] [INFO] [bran::requests] [CPU0]   [26] /boot/beeper (cmdline='init') size=33296
[13408832217] [INFO] [bran::requests] [CPU0]   [27] /boot/nectar (cmdline='init') size=176656
[13409506671] [INFO] [bran::requests] [CPU0]   [28] /boot/hello_std (cmdline='') size=16880
[13410234717] [INFO] [bran::requests] [CPU0]   [29] /boot/stdio_demo (cmdline='') size=16880
[13410869703] [INFO] [bran::requests] [CPU0]   [30] /boot/telnetd (cmdline='') size=168464
[13411537359] [INFO] [bran::requests] [CPU0]   [31] /assets/wallpapers/clouds.bmp (cmdline='') size=786486
[13412403939] [INFO] [bran::requests] [CPU0]   [32] /assets/wallpapers/flower.bmp (cmdline='') size=8386614
[13413340974] [INFO] [bran::requests] [CPU0]   [33] /assets/wallpapers/leather.bmp (cmdline='') size=1179702
[13414368231] [INFO] [bran::requests] [CPU0]   [34] /assets/wallpapers/linen.bmp (cmdline='') size=4718646
[13415305992] [INFO] [bran::requests] [CPU0]   [35] /assets/fonts/NotoSans-Regular.ttf (cmdline='') size=569208
[13416306255] [INFO] [bran::requests] [CPU0]   [36] /assets/themes/genie_circles.wasm (cmdline='') size=2824
[13417251507] [INFO] [bran::requests] [CPU0]   [37] /assets/cursors/future/default.svg (cmdline='') size=3051
[13418203656] [INFO] [bran::requests] [CPU0]   [38] /boot/locale.conf (cmdline='') size=85
[13420183260] [INFO] [kernel::memory] [CPU0] Memory map has 64 entries
[13422640506] [INFO] [kernel::memory] [CPU0]   [0] 0x0 - 0x87000 (Usable)
[13423792866] [INFO] [kernel::memory] [CPU0]   [1] 0x87000 - 0x88000 (Reserved)
[13424645982] [INFO] [kernel::memory] [CPU0]   [2] 0x88000 - 0xa0000 (Usable)
[13425499230] [INFO] [kernel::memory] [CPU0]   [3] 0x100000 - 0x800000 (Usable)
[13426316112] [INFO] [kernel::memory] [CPU0]   [4] 0x800000 - 0x808000 (Other)
[13427245029] [INFO] [kernel::memory] [CPU0]   [5] 0x808000 - 0x80b000 (Usable)
[13428098475] [INFO] [kernel::memory] [CPU0]   [6] 0x80b000 - 0x80c000 (Other)
[13428964065] [INFO] [kernel::memory] [CPU0]   [7] 0x80c000 - 0x811000 (Usable)
[13429815993] [INFO] [kernel::memory] [CPU0]   [8] 0x811000 - 0x900000 (Other)
[13430648286] [INFO] [kernel::memory] [CPU0]   [9] 0x900000 - 0x1780000 (Reserved)
[13431526911] [INFO] [kernel::memory] [CPU0]   [10] 0x1780000 - 0x7795f000 (Usable)
[13432455432] [INFO] [kernel::memory] [CPU0]   [11] 0x7795f000 - 0x779c3000 (Reserved)
[13433382402] [INFO] [kernel::memory] [CPU0]   [12] 0x779c3000 - 0x779c4000 (Other)
[13434279804] [INFO] [kernel::memory] [CPU0]   [13] 0x779c4000 - 0x779c5000 (Reserved)
[13435300461] [INFO] [kernel::memory] [CPU0]   [14] 0x779c5000 - 0x779c6000 (Other)
[13435928385] [INFO] [kernel::memory] [CPU0]   [15] 0x779c6000 - 0x779c7000 (Reserved)
[13436511033] [INFO] [kernel::memory] [CPU0]   [16] 0x779c7000 - 0x779c8000 (Other)
[13437072297] [INFO] [kernel::memory] [CPU0]   [17] 0x779c8000 - 0x779c9000 (Reserved)
[13437651084] [INFO] [kernel::memory] [CPU0]   [18] 0x779c9000 - 0x77a54000 (Other)
[13438211457] [INFO] [kernel::memory] [CPU0]   [19] 0x77a54000 - 0x77a55000 (Reserved)
[13438811331] [INFO] [kernel::memory] [CPU0]   [20] 0x77a55000 - 0x77ed6000 (Other)
[13439441499] [INFO] [kernel::memory] [CPU0]   [21] 0x77ed6000 - 0x77ed7000 (Reserved)
[13440092952] [INFO] [kernel::memory] [CPU0]   [22] 0x77ed7000 - 0x77ff8000 (Other)
[13440735924] [INFO] [kernel::memory] [CPU0]   [23] 0x77ff8000 - 0x77ff9000 (Reserved)
[13441406154] [INFO] [kernel::memory] [CPU0]   [24] 0x77ff9000 - 0x787f9000 (Other)
[13442105556] [INFO] [kernel::memory] [CPU0]   [25] 0x787f9000 - 0x787fa000 (Reserved)
[13442691669] [INFO] [kernel::memory] [CPU0]   [26] 0x787fa000 - 0x788bb000 (Other)
[13443400608] [INFO] [kernel::memory] [CPU0]   [27] 0x788bb000 - 0x788bc000 (Reserved)
[13444254450] [INFO] [kernel::memory] [CPU0]   [28] 0x788bc000 - 0x788e6000 (Other)
[13444926264] [INFO] [kernel::memory] [CPU0]   [29] 0x788e6000 - 0x788e7000 (Reserved)
[13445597385] [INFO] [kernel::memory] [CPU0]   [30] 0x788e7000 - 0x788ec000 (Other)
[13446190494] [INFO] [kernel::memory] [CPU0]   [31] 0x788ec000 - 0x788ed000 (Reserved)
[13446771492] [INFO] [kernel::memory] [CPU0]   [32] 0x788ed000 - 0x788f2000 (Other)
[13447334571] [INFO] [kernel::memory] [CPU0]   [33] 0x788f2000 - 0x788f3000 (Reserved)
[13448042289] [INFO] [kernel::memory] [CPU0]   [34] 0x788f3000 - 0x7891f000 (Other)
[13448643384] [INFO] [kernel::memory] [CPU0]   [35] 0x7891f000 - 0x78921000 (Reserved)
[13449227022] [INFO] [kernel::memory] [CPU0]   [36] 0x78921000 - 0x7892a000 (Other)
[13449784491] [INFO] [kernel::memory] [CPU0]   [37] 0x7892a000 - 0x7892c000 (Reserved)
[13450383342] [INFO] [kernel::memory] [CPU0]   [38] 0x7892c000 - 0x78934000 (Other)
[13450940712] [INFO] [kernel::memory] [CPU0]   [39] 0x78934000 - 0x78935000 (Reserved)
[13451519532] [INFO] [kernel::memory] [CPU0]   [40] 0x78935000 - 0x7893f000 (Other)
[13452170622] [INFO] [kernel::memory] [CPU0]   [41] 0x7893f000 - 0x78940000 (Reserved)
[13452752874] [INFO] [kernel::memory] [CPU0]   [42] 0x78940000 - 0x7894d000 (Other)
[13453311861] [INFO] [kernel::memory] [CPU0]   [43] 0x7894d000 - 0x7894f000 (Reserved)
[13453887975] [INFO] [kernel::memory] [CPU0]   [44] 0x7894f000 - 0x7895d000 (Other)
[13454444124] [INFO] [kernel::memory] [CPU0]   [45] 0x7895d000 - 0x7895e000 (Reserved)
[13455058155] [INFO] [kernel::memory] [CPU0]   [46] 0x7895e000 - 0x7896a000 (Other)
[13455664629] [INFO] [kernel::memory] [CPU0]   [47] 0x7896a000 - 0x7896b000 (Reserved)
[13456248729] [INFO] [kernel::memory] [CPU0]   [48] 0x7896b000 - 0x7896f000 (Other)
[13456927671] [INFO] [kernel::memory] [CPU0]   [49] 0x7896f000 - 0x78970000 (Reserved)
[13457513850] [INFO] [kernel::memory] [CPU0]   [50] 0x78970000 - 0x78980000 (Other)
[13458076698] [INFO] [kernel::memory] [CPU0]   [51] 0x78980000 - 0x78981000 (Reserved)
[13458680301] [INFO] [kernel::memory] [CPU0]   [52] 0x78981000 - 0x78a11000 (Other)
[13459269846] [INFO] [kernel::memory] [CPU0]   [53] 0x78a11000 - 0x78a12000 (Reserved)
[13459846785] [INFO] [kernel::memory] [CPU0]   [54] 0x78a12000 - 0x78a1d000 (Other)
[13460594796] [INFO] [kernel::memory] [CPU0]   [55] 0x78a1d000 - 0x78a1e000 (Reserved)
[13461277533] [INFO] [kernel::memory] [CPU0]   [56] 0x78a1e000 - 0x78a43000 (Other)
[13461886746] [INFO] [kernel::memory] [CPU0]   [57] 0x78a43000 - 0x78a44000 (Reserved)
[13462469163] [INFO] [kernel::memory] [CPU0]   [58] 0x78a44000 - 0x78a50000 (Other)
[13463031582] [INFO] [kernel::memory] [CPU0]   [59] 0x78a50000 - 0x78a51000 (Reserved)
[13463608785] [INFO] [kernel::memory] [CPU0]   [60] 0x78a51000 - 0x78a5e000 (Other)
[13464203676] [INFO] [kernel::memory] [CPU0]   [61] 0x78a5e000 - 0x78a5f000 (Reserved)
[13464786588] [INFO] [kernel::memory] [CPU0]   [62] 0x78a5f000 - 0x78aa7000 (Other)
[13465377090] [INFO] [kernel::memory] [CPU0]   [63] 0x78aa7000 - 0x78aa8000 (Reserved)
[13466287461] [INFO] [kernel::memory] [CPU0] HHDM Offset: 0xffff800000000000
[13813217649] [CONTRACT] [kernel::memory] [CPU0] Frame allocator initialized with 485751 free frames
[13824911430] [INFO] [bran::arch] [CPU0] IOAPIC: hhdm=0xffff800000000000
[13830767280] [INFO] [bran::arch] [CPU0] IOAPIC: Disabling legacy PIC...
[13832437905] [INFO] [bran::arch] [CPU0] IOAPIC: PIC disabled OK
[13833626400] [INFO] [bran::arch] [CPU0] IOAPIC: RSDP virt=0x7f77e014
[13838331804] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: total length 144
[13840663815] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77802c
[13842621903] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778034
[13843810761] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77803c
[13844877915] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f778044
[13845839964] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 1, len 12 at 0xffff80007f77804c
[13847164716] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778058
[13848214215] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778062
[13849067463] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f77806c
[13850053932] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778076
[13851119469] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778080
[13852001394] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 4, len 6 at 0xffff80007f77808a
[13853626776] [INFO] [bran::arch] [CPU0] IOAPIC: MADT parsed OK
[13854805305] [INFO] [bran::arch] [CPU0] SMP: Found 4 CPUs (CPU_COUNT now = 4)
[13855554471] [INFO] [bran::arch] [CPU0] IOAPIC: Found at phys 0xfec00000, GSI base 0
[13857450090] [INFO] [bran::arch] [CPU0] IOAPIC: Registers initialized
[13858815366] [INFO] [bran::arch] [CPU0] IOAPIC: Local APIC enabled (SVR=0x1FF, TPR=0)
[13860268488] [INFO] [bran::arch] [CPU0] IOAPIC: version 0x20, 24 redir entries
[13862538360] [INFO] [bran::arch] [CPU0] IOAPIC: All pins masked
[13864817835] [INFO] [bran::arch] [CPU0] IOAPIC: IRQ1 -> GSI 1 -> 0x21
[13865690916] [INFO] [bran::arch] [CPU0] IOAPIC: IRQ12 -> GSI 12 -> 0x2C
[13866257790] [INFO] [bran::arch] [CPU0] IOAPIC: Init complete
[13867075860] [CONTRACT] [kernel] [CPU0] Initializing global allocator...
[14318207607] [INFO] [kernel::memory::global_alloc] [CPU0] Global allocator initialized (LinkedHeap, 32MB)
[14319745869] [CONTRACT] [kernel] [CPU0] Seeding entropy pool...
[14325100713] [INFO] [kernel::entropy] [CPU0] ENTROPY: no hardware RNG available, using timer fallback (NOT seeded)
[14326682898] [INFO] [kernel::entropy] [CPU0] ENTROPY: marked seeded with weak entropy (timer-only fallback)
[14327979930] [CONTRACT] [kernel] [CPU0] Initializing SIMD...
[14329958841] [CONTRACT] [kernel] [CPU0] Initializing tasking...
[14345208405] [INFO] [kernel::sched] [CPU0]   Acquiring scheduler lock...
[14347216026] [INFO] [kernel::sched] [CPU0]   Lock acquired, checking if initialized...
[14348447190] [INFO] [kernel::sched] [CPU0]   Allocating scheduler...
[14351024160] [INFO] [kernel::sched] [CPU0]   Leaking scheduler...
[14351940273] [INFO] [kernel::sched] [CPU0]   Initializing boot task...
[14354994819] [INFO] [kernel::sched] [CPU0]   Creating boot task...
[14365416186] [INFO] [kernel::sched] [CPU0]   Creating idle tasks...
[14367769779] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 1 assigned to CPU 0
[14383855596] [INFO] [kernel::sched] [CPU0]   Boot task initialized
[14384826324] [INFO] [kernel::sched] [CPU0]   Storing scheduler pointer...
[14408516430] [INFO] [kernel::sched] [CPU0]   Initialized 4 event ring(s)
[14409614373] [CONTRACT] [kernel::sched] [CPU0] Scheduler initialized
[14415068184] [CONTRACT] [alloc] [CPU0] MEDIUM ALLOC #1: 224 KB align=8 total=0MB
[14417849259] [INFO] [kernel::task] [CPU0]   Creating graph worker tasks...
[14419819821] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 2 assigned to CPU 0
[14422875126] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 3 assigned to CPU 0
[14423855391] [INFO] [kernel] [CPU0] System initialized. Setting up preemption timer (100Hz)...
[14459455329] [INFO] [bran::arch::x86_64] [CPU0] LAPIC: calibrated timer (62640400 ticks/sec), init_cnt=626404 for 100Hz
[14461773810] [INFO] [kernel] [CPU0] Kernel: Detected 4 CPUs. Starting 3 secondaries...
[14463033585] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting 3 secondary CPUs...
[14464780737] [INFO] [bran::arch::x86_64] [CPU0] SMP: Initializing trampoline at 0x8000
[14471183859] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 1 (APIC 1)
[14502431922] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 1 (APIC 1) is online
[14503897782] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 2 (APIC 2)
[14505329421] [INFO] [kernel] [CPU1] SMP: Entering kernel_secondary_entry for CPU 1
[14506858212] [INFO] [kernel::sched] [CPU1] SMP: CPU 1 online (triggered by scheduler spawn)
[14508149865] [DEBUG] [kernel::sched::spawn] [CPU1] SCHED: Task 4 assigned to CPU 1
[14511156528] [INFO] [kernel::sched] [CPU1] SMP: Secondary CPU 1 online!
[14512602621] [INFO] [kernel::task] [CPU1] SMP: CPU 1 bootstrapped with idle task 4
[14535149475] [INFO] [kernel] [CPU2] SMP: Entering kernel_secondary_entry for CPU 2
[14536807890] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 2 (APIC 2) is online
[14538036513] [INFO] [kernel::sched] [CPU2] SMP: CPU 2 online (triggered by scheduler spawn)
[14539850358] [INFO] [bran::arch::x86_64] [CPU0] SMP: Starting CPU 3 (APIC 3)
[14545152930] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 5 assigned to CPU 2
[14547236154] [INFO] [kernel::sched] [CPU2] SMP: Secondary CPU 2 online!
[14548264764] [INFO] [kernel::task] [CPU2] SMP: CPU 2 bootstrapped with idle task 5
[14573824815] [INFO] [kernel] [CPU3] SMP: Entering kernel_secondary_entry for CPU 3
[14575433532] [INFO] [bran::arch::x86_64] [CPU0] SMP: CPU 3 (APIC 3) is online
[14576479302] [INFO] [kernel::sched] [CPU3] SMP: CPU 3 online (triggered by scheduler spawn)
[14578684296] [INFO] [bran::arch::x86_64] [CPU0] SMP: Secondary CPU startup complete
[14579502399] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 6 assigned to CPU 3
[14580403893] [INFO] [kernel] [CPU0] Kernel: Secondary CPU bring-up complete.
[14581010103] [INFO] [kernel::sched] [CPU3] SMP: Secondary CPU 3 online!
[14581740393] [INFO] [kernel::task] [CPU3] SMP: CPU 3 bootstrapped with idle task 6
[14588354649] [INFO] [kernel::root] [CPU0] Spawning Root service...
[14589581358] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 7 assigned to CPU 1
[14592513243] [INFO] [kernel::root::boot_register] [CPU0] ROOT: boot registration begin (Census Phase 1 v0.2)
[14593815192] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 1. Host creation start
[14599195314] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
RM
[14601006915] [INFO] [kernel::root::service] [CPU1] ROOT: started once
[14602476306] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Initializing components...
[14603896032] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Graph initialized
[14605033938] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Journal initialized
[14605925664] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Interner initialized
[14619973632] [CONTRACT] [kernel::root::service] [CPU1] ROOT: LogSymbols initialized
[14624875584] [CONTRACT] [kernel::root::service] [CPU1] ROOT: BatchScratch initialized
[14626733583] [CONTRACT] [kernel::root::service] [CPU1] ROOT: QueryScratch initialized
[14627933529] [CONTRACT] [kernel::root::service] [CPU1] ROOT: Entering main loop
[14672192139] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[14674929522] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[14678520285] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[14680162629] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[14682480879] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[14684902551] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[14686860045] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 1. Host registered: t1
[14687642178] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 2. Platform Bus start
[14688726426] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[14695219737] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[14697005433] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[14699709288] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[14702628567] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[14705265069] [DEBUG] [kernel::sched::sleep] [CPU1] DIAG ctx_switch: cpu=1 from=4 to=7
[14708186592] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 2. Platform Bus done
[14709000504] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 3. Kernel start
[14726389854] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 3. Kernel done
[14727362100] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 4. Root Service start
[14737023675] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 4. Root Service done
[14738281602] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 5. CPUs start
[14775275097] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 5. CPUs done
[14776553649] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 6. Memory Ranges start
[15160014342] [CONTRACT] [alloc] [CPU1] MEDIUM ALLOC #2: 120 KB align=8 total=1MB
[15718296627] [INFO] [kernel::root::boot_register] [CPU0] ROOT_DIAG: 6. Memory Ranges done
[15752424171] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=1000 nodes=331 watches=0 history=498 journal=423 symbols=51 drops=0
[15789917385] [CONTRACT] [alloc] [CPU1] MEDIUM ALLOC #3: 240 KB align=8 total=1MB
[17417584437] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=2000 nodes=446 watches=0 history=959 journal=768 symbols=93 drops=0
[18286651713] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: filesystem bootstrap: /initrd/ with 39 files
[18461940255] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: Census Phase 2: PCI
[18463759149] [INFO] [kernel::root::pci] [CPU0] PCI: Starting enumeration...
[18576584499] [INFO] [kernel::root::pci] [CPU0] PCI: 00:00.0 8086:29c0 Intel Corporation 82G33/G31/P35/P31 Express DRAM Controller class=06:00 prog_if=00 rev=00
[18643993896] [INFO] [kernel::root::pci] [CPU0] PCI: 00:01.0 1af4:1050 Red Hat, Inc. Virtio 1.0 GPU class=03:00 prog_if=00 rev=01
[18692921709] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR0: phys=0x80000000 size=0x800000
[18694251906] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR2: phys=0xc000000000 size=0x4000
[18695625564] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR4: phys=0x80882000 size=0x1000
[18702951069] [INFO] [kernel::root::pci] [CPU0] PCI: Found virtio display controller at 00:01.0
[18727048032] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO notify_cfg BAR2 offset=0x3000 mult=4
[18745213047] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO common_cfg BAR2 offset=0x1000
[18747591159] [INFO] [kernel::root::pci] [CPU0] PCI: Registered virtio GPU (graph_id=1086, idx=1) BAR0=0x80000000
[18808565259] [INFO] [kernel::root::pci] [CPU0] PCI: 00:02.0 1af4:1000 Red Hat, Inc. Virtio network device class=02:00 prog_if=00 rev=00
[18827392254] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR1: phys=0x80881000 size=0x1000
[18828329388] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR4: phys=0xc000004000 size=0x4000
[18833482008] [INFO] [kernel::root::pci] [CPU0] PCI: Found virtio network controller at 00:02.0
[18858307743] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO notify_cfg BAR4 offset=0x3000 mult=4
[18878017422] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO common_cfg BAR4 offset=0x0
[18882491826] [INFO] [kernel::root::pci] [CPU0] PCI: Registered virtio network (graph_id=1120, idx=2) BAR4=0xc000004000
[18883680882] [INFO] [kernel::root::pci] [CPU0] VirtIO-Net: Device registered for userspace driver
[18962605893] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.0 8086:2918 Intel Corporation 82801IB (ICH9) LPC Interface Controller class=06:01 prog_if=00 rev=02
[18964624965] [INFO] [kernel::root::pci] [CPU0] PCI: Found LPC/ISA bridge at 00:1f.0
[19059445977] [INFO] [kernel::root::pci] [CPU0] LPC: Created Legacy IO bus with CMOS and PS/2 controller
[19133303970] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.2 8086:2922 Intel Corporation 82801IR/IO/IH (ICH9R/DO/DH) 6 port SATA Controller [AHCI mode] class=01:06 prog_if=01 rev=02
[19145906967] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR5: phys=0x80880000 size=0x1000
[19150030020] [INFO] [kernel::root::pci] [CPU0] PCI: Found AHCI SATA controller at 00:1f.2
[19152224718] [INFO] [kernel::root::pci] [CPU0] PCI: Registered AHCI controller (graph_id=1193, idx=5) BAR5=0x80880000
[19154431956] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=3000 nodes=583 watches=0 history=1024 journal=1024 symbols=181 drops=0
[19243819980] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.3 8086:2930 Intel Corporation 82801I (ICH9 Family) SMBus Controller class=0c:05 prog_if=00 rev=02
[19250054208] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: registered items. host=t1 kernel=tb
[19251197229] [CONTRACT] [kernel] [CPU0] KERNEL: root census complete: host=t1 kernel=tb root=te
[19252174491] [CONTRACT] [kernel] [CPU0] Kernel: Enumerating 39 boot modules...
[19253091990] [CONTRACT] [kernel] [CPU0]   [0] name='/boot/sprout' cmdline='init' size=111120
[19253778258] [CONTRACT] [kernel] [CPU0]   [1] name='/boot/bristle' cmdline='init' size=41456
[19254438522] [CONTRACT] [kernel] [CPU0]   [2] name='/boot/rtc_cmos' cmdline='init' size=29248
[19255128420] [CONTRACT] [kernel] [CPU0]   [3] name='/boot/clock' cmdline='' size=61968
[19255840164] [CONTRACT] [kernel] [CPU0]   [4] name='/boot/taskman' cmdline='' size=70160
[19256713245] [CONTRACT] [kernel] [CPU0]   [5] name='/boot/ps2_kbd' cmdline='init' size=25072
[19257370473] [CONTRACT] [kernel] [CPU0]   [6] name='/boot/echo' cmdline='init' size=33264
[19258002390] [CONTRACT] [kernel] [CPU0]   [7] name='/boot/bloom' cmdline='init' size=969224
[19258672356] [CONTRACT] [kernel] [CPU0]   [8] name='/boot/ps2_mouse' cmdline='init' size=29168
[19259512371] [CONTRACT] [kernel] [CPU0]   [9] name='/boot/display_bootfb' cmdline='init' size=29248
[19260213753] [CONTRACT] [kernel] [CPU0]   [10] name='/boot/display_virtio_gpu' cmdline='init' size=53864
[19260947244] [CONTRACT] [kernel] [CPU0]   [11] name='/boot/fontd' cmdline='init' size=188944
[19261733601] [CONTRACT] [kernel] [CPU0]   [12] name='/boot/blossom' cmdline='init' size=123408
[19262587740] [CONTRACT] [kernel] [CPU0]   [13] name='/boot/flytrap' cmdline='' size=291352
[19263226191] [CONTRACT] [kernel] [CPU0]   [14] name='/boot/virtio_netd' cmdline='' size=49680
[19263884706] [CONTRACT] [kernel] [CPU0]   [15] name='/boot/rtl8168d' cmdline='' size=45584
[19264522167] [CONTRACT] [kernel] [CPU0]   [16] name='/boot/netd' cmdline='' size=147984
[19265142072] [CONTRACT] [kernel] [CPU0]   [17] name='/boot/fetchd' cmdline='' size=41488
[19265793426] [CONTRACT] [kernel] [CPU0]   [18] name='/boot/anther' cmdline='' size=586272
[19266455934] [CONTRACT] [kernel] [CPU0]   [19] name='/boot/fortune' cmdline='' size=61968
[19267094121] [CONTRACT] [kernel] [CPU0]   [20] name='/boot/photosynthesis' cmdline='' size=12648
[19267932486] [CONTRACT] [kernel] [CPU0]   [21] name='/boot/ahci_disk' cmdline='' size=45664
[19268615553] [CONTRACT] [kernel] [CPU0]   [22] name='/boot/iso9660d' cmdline='' size=53856
[19269257007] [CONTRACT] [kernel] [CPU0]   [23] name='/boot/virtio_sound' cmdline='init' size=49648
[19269952680] [CONTRACT] [kernel] [CPU0]   [24] name='/boot/hdaudio' cmdline='' size=37360
[19270601130] [CONTRACT] [kernel] [CPU0]   [25] name='/boot/pci_stubd' cmdline='' size=29168
[19271250570] [CONTRACT] [kernel] [CPU0]   [26] name='/boot/beeper' cmdline='init' size=33296
[19271926014] [CONTRACT] [kernel] [CPU0]   [27] name='/boot/nectar' cmdline='init' size=176656
[19272588060] [CONTRACT] [kernel] [CPU0]   [28] name='/boot/hello_std' cmdline='' size=16880
[19273434543] [CONTRACT] [kernel] [CPU0]   [29] name='/boot/stdio_demo' cmdline='' size=16880
[19274388111] [CONTRACT] [kernel] [CPU0]   [30] name='/boot/telnetd' cmdline='' size=168464
[19275558555] [CONTRACT] [kernel] [CPU0]   [31] name='/assets/wallpapers/clouds.bmp' cmdline='' size=786486
[19276483479] [CONTRACT] [kernel] [CPU0]   [32] name='/assets/wallpapers/flower.bmp' cmdline='' size=8386614
[19277353458] [CONTRACT] [kernel] [CPU0]   [33] name='/assets/wallpapers/leather.bmp' cmdline='' size=1179702
[19278127143] [CONTRACT] [kernel] [CPU0]   [34] name='/assets/wallpapers/linen.bmp' cmdline='' size=4718646
[19279059723] [CONTRACT] [kernel] [CPU0]   [35] name='/assets/fonts/NotoSans-Regular.ttf' cmdline='' size=569208
[19279854330] [CONTRACT] [kernel] [CPU0]   [36] name='/assets/themes/genie_circles.wasm' cmdline='' size=2824
[19280614782] [CONTRACT] [kernel] [CPU0]   [37] name='/assets/cursors/future/default.svg' cmdline='' size=3051
[19281377775] [CONTRACT] [kernel] [CPU0]   [38] name='/boot/locale.conf' cmdline='' size=85
[19283039226] [INFO] [kernel] [CPU0] Found init module: /boot/sprout (cmdline: 'init'), loading...
[19284833073] [INFO] [kernel::task::loader] [CPU0] Loading module: /boot/sprout
[19285858911] [DEBUG] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19294797225] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[19310787243] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=213000 exec=false
[19315990056] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=21a000 exec=false
[19326236457] [INFO] [kernel] [CPU0] Spawning sprout with registry at 0x600000...
[19326985887] [CONTRACT] [kernel] [CPU0] Spawning init process...
[19329188142] [DEBUG] [kernel::sched::spawn] [CPU0] SCHED: Task 8 (user task/process) assigned to CPU 2
[19332737457] [CONTRACT] [kernel] [CPU0] Entering scheduler loop.
[19333465701] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600000
ThingOS Petals
type 'help' for commands

petals> [19338554763] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=8 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369013504 RFLAGS_BEFORE=134 CR3_BEFORE=50393088 fs_base=0 gs_base=18446744071564586608
[19342789554] [DEBUG] [kernel::sched::sleep] [CPU0] DIAG ctx_switch: cpu=0 from=0 to=2
[19357872138] [INFO] [sprout] [CPU2] [sprout] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ff990 rip=0x20139f rflags=0x206
[19362949122] [INFO] [sprout] [CPU2] SPROUT: v0.4 starting (Supervisor Mode)...
[19364491377] [INFO] [sprout::devtree] [CPU2] SPROUT: devtree::init entry (v0.2)
[19365811674] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 1: Find Host
[19374374712] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 2: HHDM
[19379771697] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 3: Platform Bus
[19383725757] [INFO] [sprout::devtree] [CPU2] SPROUT: Step 4: Firmware
[19384932765] [INFO] [sprout::devtree] [CPU2] SPROUT: Finding ACPI...
[19388977971] [INFO] [sprout::devtree] [CPU2] SPROUT: Found 1 ACPI nodes
[19392845010] [INFO] [sprout::devtree] [CPU2] SPROUT: ACPI RSDP = 0x7f77e014
[19394758119] [INFO] [sprout::devtree] [CPU2] SPROUT: Finding DTB...
[19400344161] [INFO] [sprout::devtree] [CPU2] SPROUT: Found 0 DTB nodes
[19402038711] [INFO] [sprout::devtree] [CPU2] SPROUT: Init OK, returning context
[19403854866] [INFO] [sprout::devtree] [CPU2] SPROUT: build() called
[19405649010] [INFO] [sprout::devtree::x86_64] [CPU2] SPROUT: x86_64 platform enrichment... (v0.2)
[19410530271] [INFO] [sprout::devtree::x86_64] [CPU2] SPROUT: x86_64 enumerate done
[19412567130] [INFO] [sprout] [CPU2] SPROUT: About to create Supervisor...
[19414596102] [INFO] [sprout] [CPU2] SPROUT: Supervisor created, calling run_forever...
[19417144527] [INFO] [sprout::supervisor] [CPU2] SPROUT: Supervisor starting (phased mode)...
[19419697308] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 1] Hardware Discovery and Core Drivers
[19423016712] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovering modules...
[19429215531] [INFO] [sprout::supervisor] [CPU2] SPROUT: Found 39 modules
[19483661604] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[0] = '/boot/sprout'
[19492433202] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[1] = '/boot/bristle'
[19498239882] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[2] = '/boot/rtc_cmos'
[19502859453] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[3] = '/boot/clock'
[19506058242] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/clock
[19509715830] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[4] = '/boot/taskman'
[19515494691] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[5] = '/boot/ps2_kbd'
[19520560818] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[6] = '/boot/echo'
[19525501644] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[7] = '/boot/bloom'
[19530583314] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[8] = '/boot/ps2_mouse'
[19535988351] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[9] = '/boot/display_bootfb'
[19542485391] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[10] = '/boot/display_virtio_gpu'
[19548661869] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[11] = '/boot/fontd'
[19558307439] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[12] = '/boot/blossom'
[19565356239] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[13] = '/boot/flytrap'
[19572208557] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[14] = '/boot/virtio_netd'
[19580021340] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[15] = '/boot/rtl8168d'
[19587351366] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[16] = '/boot/netd'
[19594253613] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[17] = '/boot/fetchd'
[19600916577] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[18] = '/boot/anther'
[19607824137] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[19] = '/boot/fortune'
[19614974148] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[20] = '/boot/photosynthesis'
[19620817326] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[21] = '/boot/ahci_disk'
[19627693767] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[22] = '/boot/iso9660d'
[19634655645] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[23] = '/boot/virtio_sound'
[19641369825] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[24] = '/boot/hdaudio'
[19647834063] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[25] = '/boot/pci_stubd'
[19653968730] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[26] = '/boot/beeper'
[19661340567] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[27] = '/boot/nectar'
[19669415568] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[28] = '/boot/hello_std'
[19675886307] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/hello_std
[19681846800] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[29] = '/boot/stdio_demo'
[19688689317] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[30] = '/boot/telnetd'
[19693033404] [INFO] [sprout::supervisor] [CPU2] SPROUT: Discovered app: /boot/telnetd
[19697083098] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[31] = '/assets/wallpapers/clouds.bmp'
[19704838164] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[32] = '/assets/wallpapers/flower.bmp'
[19711280523] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[33] = '/assets/wallpapers/leather.bmp'
[19716896430] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[34] = '/assets/wallpapers/linen.bmp'
[19722882465] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[35] = '/assets/fonts/NotoSans-Regular.ttf'
[19728821211] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[36] = '/assets/themes/genie_circles.wasm'
[19734136356] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[37] = '/assets/cursors/future/default.svg'
[19742182515] [INFO] [sprout::supervisor] [CPU2] SPROUT: Module[38] = '/boot/locale.conf'
[19747119282] [INFO] [sprout::registry] [CPU2] SPROUT: Scanning boot modules...
[19771431900] [INFO] [sprout::registry] [CPU2] SPROUT: Registering driver 'dev.rtc.Cmos' -> '/boot/rtc_cmos' (fallback)
[19951644768] [INFO] [sprout::registry] [CPU2] SPROUT: Registry scan complete. Found 1 drivers.
[19959621264] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/pci_stubd
[19960510944] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19962182493] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[19967257101] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[19968863343] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[19978972167] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 9 (user task/process) assigned to CPU 3
[19984221576] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db400
[19986018954] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned pci_stubd (PID=9)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[19988170389] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=9 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369079040 RFLAGS_BEFORE=134 CR3_BEFORE=50860032 fs_base=0 gs_base=18446744071564586640
[19997644425] [INFO] [pci_stubd] [CPU3] pci_stubd: starting pci-id matcher
[20022050070] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/rtc_cmos
[20026141410] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[20028681321] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20042145750] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=204000 exec=false
[20045720442] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[20060595621] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 10 (user task/process) assigned to CPU 1
[20076487761] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned rtc_cmos (PID=10)
[20080322361] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up storage pipeline...
[20098561329] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x49b
[20105382297] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=10 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369144576 RFLAGS_BEFORE=134 CR3_BEFORE=50974720 fs_base=0 gs_base=18446744071564586576
[20118229890] [INFO] [rtc_cmos] [CPU1] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ffeb0 rip=0x201027 rflags=0x202
[20119987107] [INFO] [rtc_cmos] [CPU1] Starting... arg=49b
[20122338885] [INFO] [rtc_cmos] [CPU1] Serving device ID: ThingId([155, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[20128719501] [INFO] [rtc_cmos] [CPU1] RTC: 2026-04-06 02:25:29 = 1775442329 unix_secs
[20131348149] [INFO] [kernel::time] [CPU1] System clock anchored: unix_secs=1775442329, mono_ns=10065328977, offset=1775442318934671023ns
[20133696000] [INFO] [rtc_cmos] [CPU1] System clock anchored
[20148930021] [INFO] [rtc_cmos] [CPU1] RTC: Set sys.TimeState = 1 (Anchored)
[20196736692] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ahci_disk
[20197888788] [INFO] [rtc_cmos] [CPU1] Publishing time. Entering maintenance loop.
[20199004320] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[20200949076] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20207461692] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[20210679225] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20a000 exec=false
[20220418086] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 11 (user task/process) assigned to CPU 2
[20224194936] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ahci_disk (PID=11)
[20226698547] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db400
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[20228552883] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=11 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369210784 RFLAGS_BEFORE=134 CR3_BEFORE=51089408 fs_base=0 gs_base=18446744071564586608
[20234542053] [INFO] [ahci_disk] [CPU2] AHCI: Starting AHCI/SATA disk driver v1
[20239890297] [INFO] [sprout::pipelines] [CPU2] SPROUT: Failed to spawn ata_disk: ENOENT
[20241959298] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/iso9660d
[20242865280] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[20244776079] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20252729376] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[20255189196] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[20263513347] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 12 (user task/process) assigned to CPU 3
[20267893305] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db400
[20269512615] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned iso9660d (PID=12)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[20271336195] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=12 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369277376 RFLAGS_BEFORE=134 CR3_BEFORE=51228672 fs_base=0 gs_base=18446744071564586640
[20275899996] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up audio driver...
[20278973055] [INFO] [iso9660d] [CPU3] ISO9660D: Starting ISO9660 mount service...
[20282856957] [INFO] [ahci_disk] [CPU2] AHCI: Found 6 PCI functions
[20291532591] [INFO] [iso9660d] [CPU3] ISO9660D: Found 0 block devices
[20292652875] [INFO] [sprout::pipelines] [CPU2] SPROUT: No Sound device found
[20293734384] [INFO] [iso9660d] [CPU3] ISO9660D: No ISO9660 filesystems found
[20294792430] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up display pipeline...
[20295800679] [INFO] [iso9660d] [CPU3] ISO9660D: Entering service loop with 0 mounts
[20314711527] [INFO] [sprout::pipelines] [CPU2] SPROUT: Using VirtIO GPU at 1920x1080
[20316583353] [INFO] [sprout::pipelines] [CPU2] SPROUT: Display backend: VirtIO-GPU (1920x1080 stride=7680)
[20837665068] [INFO] [ahci_disk] [CPU2] AHCI: Found AHCI controller at PCI func ThingId([169, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[20843006613] [INFO] [ahci_disk] [CPU2] AHCI: BAR5=0x80880000
[20846664333] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 11 claimed device 1193 (handle 0)
[20848382841] [INFO] [ahci_disk] [CPU2] AHCI: Claimed PCI device ThingId([169, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) handle=0
[20852754252] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR5 phys=0x80880000 size=0x1000 -> virt=0x10000000
[20854562157] [INFO] [ahci_disk] [CPU2] AHCI: Mapped ABAR at 0x10000000
[20856858858] [INFO] [ahci_disk] [CPU2] AHCI: Version 1.0, 6 ports, 32 slots, 64-bit: true
[20858636106] [INFO] [ahci_disk] [CPU2] AHCI: Ports implemented: 0x3f
[20860066194] [INFO] [ahci_disk] [CPU2] AHCI: DMA virt=0x20b000
[20862984681] [INFO] [ahci_disk] [CPU2] AHCI: DMA phys=0x30c8000
[20864500305] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 0...
[20865933693] [INFO] [ahci_disk] [CPU2] AHCI: Port 0 - no device
[20867420805] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 1...
[20868893595] [INFO] [ahci_disk] [CPU2] AHCI: Port 1 - no device
[20870242470] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 2...
[20871723312] [INFO] [ahci_disk] [CPU2] AHCI: Port 2 - SATAPI drive (sig=0xeb140101)
[20877662982] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/display_virtio_gpu
[20878813527] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[20880709707] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[20888160117] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[20890956933] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[20898566865] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 13 (user task/process) assigned to CPU 1
[20901944151] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db400
[20903635137] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned display driver '/display_virtio_gpu' (PID=13)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x30002
[20905621572] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=13 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369352816 RFLAGS_BEFORE=130 CR3_BEFORE=59666432 fs_base=0 gs_base=18446744071564586576
[20912855634] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: starting (drv_req_r=2, drv_resp_w=3)
[20921940930] [INFO] [kernel::syscall::handlers::device] [CPU1] DEVICE: task 13 claimed device 1108 (handle 1)
[20943856461] [INFO] [ahci_disk] [CPU2] AHCI: Registered ATAPI block device 1235 port=2 model='                                        ' rpc_port=5
[20951368911] [INFO] [virtio_gpu] [CPU1] virtio_gpu: caps from graph - common BAR2 off=0x1000, notify BAR2 off=0x3000 mult=4
[20953666998] [INFO] [kernel::syscall::handlers::device] [CPU1] DEVICE: Mapped BAR2 phys=0xc000000000 size=0x4000 -> virt=0x10001000
[20959379958] [INFO] [virtio_gpu] [CPU1] virtio_gpu: device features=0x30000002 virgl=false
[20968715394] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: GPU initialized successfully
[20970567651] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: Virgl 3D not supported, using 2D only
[20976222399] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 3...
[20977774785] [INFO] [ahci_disk] [CPU2] AHCI: Port 3 - no device
[20979169893] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 4...
[20980138608] [INFO] [ahci_disk] [CPU2] AHCI: Port 4 - no device
[20981357991] [INFO] [ahci_disk] [CPU2] AHCI: Probing port 5...
[20982647103] [INFO] [ahci_disk] [CPU2] AHCI: Port 5 - no device
[20983993272] [INFO] [ahci_disk] [CPU2] AHCI: Found 1 SATA disk(s)
[20985378117] [INFO] [ahci_disk] [CPU2] AHCI: Entering RPC service loop
[20988981585] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: creating frame pool 1x 1920x1080 stride=7680 format=2
[20992410549] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up input pipeline (keyboard + mouse)...
[20995333326] [INFO] [sprout::pipelines] [CPU2] SPROUT: Created kbd_raw port (w=7, r=8)
[20998218186] [INFO] [sprout::pipelines] [CPU2] SPROUT: Created mouse_raw port (w=9, r=10)
[21000567456] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ps2_kbd
[21001546368] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21003541416] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21007495311] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=203000 exec=false
[21009555336] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[21017466690] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 14 (user task/process) assigned to CPU 2
[21020412039] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ps2_kbd (PID=14)
[21022672209] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/ps2_mouse
[21023586705] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21025481004] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21030561321] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[21032317350] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[21041005491] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 15 (user task/process) assigned to CPU 3
[21044801250] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb0106680
[21046109007] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned ps2_mouse (PID=15)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x9
[21048138969] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=15 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369500256 RFLAGS_BEFORE=134 CR3_BEFORE=68501504 fs_base=0 gs_base=18446744071564586640
[21052662312] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/bristle
[21053912682] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21056225883] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21062296167] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[21064152615] [INFO] [ps2_mouse] [CPU3] ps2_mouse: online (handle=9)
[21065607684] [INFO] [ps2_mouse] [CPU3] ps2_mouse: enabling aux port
[21066457797] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[21071331600] [INFO] [ps2_mouse] [CPU3] ps2_mouse: controller cfg already correct (0x47)
[21073182339] [INFO] [ps2_mouse] [CPU3] ps2_mouse: sending RESET (0xFF)
[21075438351] [INFO] [ps2_mouse] [CPU3] ps2_mouse: reset ACK received (0xfa)
[21076530057] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 16 (user task/process) assigned to CPU 1
[21078654927] [INFO] [ps2_mouse] [CPU3] ps2_mouse: BAT passed (0xaa), ID 0x00 confirmed
[21080897541] [INFO] [ps2_mouse] [CPU3] ps2_mouse: setting sample rate (100)
[21082052574] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned bristle (PID=16)
[21083662644] [INFO] [ps2_mouse] [CPU3] ps2_mouse: setting resolution (3)
[21084807480] [INFO] [sprout::pipelines] [CPU2] SPROUT: Input broker ready (keyboard + mouse)
[21087134772] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up network stack...
[21089659800] [INFO] [ps2_mouse] [CPU3] ps2_mouse: status result = Some(0) Some(3) Some(100)
[21091699332] [INFO] [ps2_mouse] [CPU3] ps2_mouse: sending enable command (0xF4)
[21092764704] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00fff90
[21094365039] [INFO] [ps2_mouse] [CPU3] ps2_mouse: enable ACK received (0xfa)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x7
[21096317319] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=14 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369434176 RFLAGS_BEFORE=130 CR3_BEFORE=68390912 fs_base=0 gs_base=18446744071564586608
[21105045852] [INFO] [ps2_kbd] [CPU2] ps2_kbd: online (handle=7)
[21460287453] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb0106680
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x8000a000b000d
[21463083312] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=16 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369583040 RFLAGS_BEFORE=134 CR3_BEFORE=68616192 fs_base=0 gs_base=18446744071564586576
[21483080454] [INFO] [bristle] [CPU1] BRISTLE_MAIN_ENTERED_WITH_LOGS_YAY
[21484180641] [INFO] [ps2_kbd] [CPU2] ps2_kbd: created driver node 1240
[21486004386] [INFO] [bristle] [CPU1] bristle: online (kbd=8, mouse=10, evt=11, echo=13)
[21487506084] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task subscribed to vector 0x21
[21490039560] [INFO] [ps2_kbd] [CPU2] ps2_kbd: subscribed to IRQ1 (vector 0x21)
[21491224425] [INFO] [bristle] [CPU1] bristle: created broadcast topic 0
[21498026154] [INFO] [sprout::pipelines] [CPU2] SPROUT: Found NIC device ThingId([117, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[21502362519] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/virtio_netd
[21503529333] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21506203455] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21507646083] [INFO] [bristle] [CPU1] bristle: registered in graph as svc.Input (id=1241)
[21517485033] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=208000 exec=false
[21520830606] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20b000 exec=false
[21530419350] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 17 (user task/process) assigned to CPU 2
[21533485314] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned virtio_netd (PID=17)
[21536055189] [INFO] [sprout::pipelines] [CPU2] SPROUT: spawn_netd start
[21537822108] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/netd
[21538818048] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21540811281] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[21561117996] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=21d000 exec=false
[21566106408] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=223000 exec=false
[21601562730] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: frame pool ready (1 buffer)
[21683722005] [INFO] [ps2_mouse] [CPU3] ps2_mouse: init done
[21684718836] [INFO] [kernel::syscall::handlers::device] [CPU3] DEVICE: task subscribed to vector 0x2c
[21686261256] [INFO] [ps2_mouse] [CPU3] ps2_mouse: subscribed to IRQ12 (vector 0x2c)
[21687894888] [INFO] [ps2_mouse] [CPU3] ps2_mouse: using cooperative polling loop (2ms interval)
[22515275541] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 18 (user task/process) assigned to CPU 3
[22519161060] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010a1b8
[22520348664] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned netd (PID=18)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[22521937647] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=18 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369720064 RFLAGS_BEFORE=134 CR3_BEFORE=68882432 fs_base=0 gs_base=18446744071564586640
[22529610114] [INFO] [netd] [CPU3] NETD: Starting network stack service...
[22531802007] [INFO] [netd] [CPU3] NETD: Looking for virtio_netd driver service...
[22533133359] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00fff90
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x475
[22536328287] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=17 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369654528 RFLAGS_BEFORE=134 CR3_BEFORE=68743168 fs_base=0 gs_base=18446744071564586608
[22545362862] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Starting VirtIO-NET driver service...
[22547900199] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Searching for NIC device...
[22553565540] [INFO] [ps2_kbd] [CPU2] ps2_kbd: using cooperative polling loop (2ms interval)
[22560427197] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Found NIC device t475
[22562897577] [INFO] [virtio::device] [CPU2] VirtIO: device::new(0x475) - claiming...
[22564693701] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: task 17 claimed device 1141 (handle 2)
[22567514310] [INFO] [virtio::device] [CPU2] VirtIO: claimed, handle=2
[22574653332] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 2] Starting Network Apps and Services
[22577701278] [INFO] [sprout::pipelines] [CPU2] SPROUT: Setting up network apps...
[22580606037] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/anther
[22582183965] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[22585010877] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[22658028030] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=268000 exec=false
[22683602535] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=28e000 exec=false
[22691713869] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 19 (user task/process) assigned to CPU 1
[22695374394] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00fff90
[22696785243] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned anther (PID=19)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[22698487383] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=19 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369785600 RFLAGS_BEFORE=134 CR3_BEFORE=79892480 fs_base=0 gs_base=18446744071564586576
[22702578228] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/nectar
[22703541531] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[22705583868] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[22706804505] [INFO] [anther] [CPU1] anther: Starting HTTP server (ThingOS anther v0.1)
[22711076718] [INFO] [anther] [CPU1] anther: Starting server mode on port 80...
[22717923690] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[22720159506] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[22733415705] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=226000 exec=false
[22737295581] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=22a000 exec=false
[22746884226] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 20 (user task/process) assigned to CPU 2
[22750900392] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned nectar (PID=20)
[22753826931] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/fetchd
[22755321765] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[22757810988] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[22764346572] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=206000 exec=false
[22767416562] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[22776805854] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 21 (user task/process) assigned to CPU 3
[22782295404] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned fetchd (PID=21)
[22783963653] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010b5a8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[22786089810] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=21 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369916672 RFLAGS_BEFORE=134 CR3_BEFORE=80826368 fs_base=0 gs_base=18446744071564586640
[22794200682] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/clock
[22795549689] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[22798016538] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[22799720757] [INFO] [fetchd] [CPU3] FETCHD: Starting IP address display...
[22802008812] [INFO] [fetchd] [CPU3] FETCHD: Waiting for UI Root (Compositor)...
[22810568583] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[22817689488] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20e000 exec=false
[22828875399] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 22 (user task/process) assigned to CPU 1
[22834554534] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned clock (PID=22)
[22835792496] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb010b5a8
[22838202156] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/taskman
[22839143580] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[22841294520] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[22848991044] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=22 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369982208 RFLAGS_BEFORE=134 CR3_BEFORE=80953344 fs_base=0 gs_base=18446744071564586576
[22853095980] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20c000 exec=false
[22858056078] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=210000 exec=false
[22868316141] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 23 (user task/process) assigned to CPU 2
[22882609200] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned taskman (PID=23)
[22886253489] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/fontd
[22887400536] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[22889973942] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[22926306183] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=226000 exec=false
[22931442501] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=22d000 exec=false
[22938916506] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 24 (user task/process) assigned to CPU 3
[22942590594] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned fontd (PID=24)
[22943498226] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb010d548
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[22946045265] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/blossom
[22946896797] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=24 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370114096 RFLAGS_BEFORE=134 CR3_BEFORE=81256448 fs_base=0 gs_base=18446744071564586640
[22951808121] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[22955471946] [INFO] [fontd] [CPU3] FONTD: Starting Graph-Native Font Service v2 (Atlas-based IPC)
[22956478050] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[22976996427] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=219000 exec=false
[22980497562] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=21d000 exec=false
[22982909961] [INFO] [fontd] [CPU3] FONTD: Service node created, req=15, resp=18
[22988764161] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 25 (user task/process) assigned to CPU 1
[22991731125] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00f10e8
[22993279551] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned blossom (PID=25)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[22995270177] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=25 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370202416 RFLAGS_BEFORE=134 CR3_BEFORE=81530880 fs_base=0 gs_base=18446744071564586576
[23000120616] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/flytrap
[23001085734] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[23002544037] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[23003971386] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[23005292706] [INFO] [blossom] [CPU1] BLOSSOM: Starting SVG Cache Service
[23006445330] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[23007964056] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[23008873107] [INFO] [blossom] [CPU1] BLOSSOM: Init UI pipeline...
[23010323787] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=225 pred=0 subj_lo=0
[23014120140] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[23015278638] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[23016719649] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[23018104956] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=226 subj_lo=0
[23031527508] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[23032955022] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[23034258423] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[23035646403] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=230 pred=0 subj_lo=0
[23052753339] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=237000 exec=false
[23064778539] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=246000 exec=false
[23075041242] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 26 (user task/process) assigned to CPU 2
[23077070742] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[23078441265] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[23080317150] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[23084935830] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned flytrap (PID=26)
[23088377730] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=231 subj_lo=0
[23093337762] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00fff90
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[23095793820] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=20 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072369851136 RFLAGS_BEFORE=134 CR3_BEFORE=80564224 fs_base=0 gs_base=18446744071564586608
[23102012868] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[23102960331] [INFO] [nectar] [CPU2] NECTAR: Started.
[23103497439] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[23105908221] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb010b5a8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[23107453380] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=23 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370048016 RFLAGS_BEFORE=130 CR3_BEFORE=81100800 fs_base=0 gs_base=18446744071564586608
[23113247652] [INFO] [taskman] [CPU2] TASKMAN: starting task manager
[23115399186] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f9fb0
[23116650909] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[23117926722] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[23119767198] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=0 start_seq=0
[23121156663] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[23123040138] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00db4e0
[23124263118] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=232 pred=0 subj_lo=0
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[23126479728] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[23128034292] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=26 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370269296 RFLAGS_BEFORE=130 CR3_BEFORE=81739776 fs_base=0 gs_base=18446744071564586608
[23131884039] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=234 subj_lo=0
[23146164624] [INFO] [fontd] [CPU3] FONTD: Opened ASSET watch (handle=1248) for kind 'Asset'
[23147855148] [INFO] [fontd] [CPU3] FONTD: Service ready
[23148797199] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 3] Starting Compositor
[23155723107] [INFO] [flytrap] [CPU2] FLYTRAP: Starting unified content provider service...
[23158384128] [INFO] [flytrap] [CPU2] FLYTRAP: Service contract validated - graph-native asset watcher
[23161457352] [INFO] [flytrap] [CPU2] FLYTRAP: Initializing Limine module content source...
[23183738820] [INFO] [sprout::pipelines] [CPU2] SPROUT: Writing bloom BS: drv_req=1, drv_resp=4, bristle_evt=12
[23191923909] [INFO] [virtio::device] [CPU2] VirtIO: common_bar=4 common_off=0x0 notify_bar=4 notify_off=0x3000 mult=4
[23200889646] [INFO] [sprout::pipelines] [CPU2] SPROUT: Bloom handles via BS=1251 backend=VirtIO-GPU
[23203054941] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/bloom
[23203957425] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[23206077477] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[23206895349] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[23208233169] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[23209514427] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[23211234255] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=239 subj_lo=0
[23220933945] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[23222639187] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[23224574307] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[23226551172] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=240 subj_lo=0
[23239047084] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[23241122124] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[23244460734] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[23246129973] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[23248023216] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[23249401956] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=241 subj_lo=0
[23258438478] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: ptr=0x7fbfb0
[23259676605] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: validating range len=48
[23261019144] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: copyin success. mode=1 start_seq=0
[23262451014] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU1] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=242 pred=0 subj_lo=0
[23301166515] [INFO] [blossom] [CPU1] BLOSSOM: UI pipeline ready
[23324869194] [INFO] [blossom] [CPU1] BLOSSOM: Service node created, req=19, resp=22
[23326408050] [INFO] [blossom] [CPU1] BLOSSOM: Service ready
[23360664888] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=2d5000 exec=false
[23376477630] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=2eb000 exec=false
[23384539134] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 27 (user task/process) assigned to CPU 3
[23387723370] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb00db6a0
[23388807189] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned bloom (PID=27)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x4e3
[23390335815] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=27 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370358352 RFLAGS_BEFORE=134 CR3_BEFORE=82391040 fs_base=0 gs_base=18446744071564586640
[23397694023] [INFO] [bloom::logging] [CPU3] bloom: logging initialized
[23399031777] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/echo
[23399871528] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[23401464240] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[23406369426] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[23408368038] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[23416455051] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 28 (user task/process) assigned to CPU 1
[23419521741] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00db6a0
[23421084324] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned echo (PID=28)
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xe
[23422988424] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=28 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370423888 RFLAGS_BEFORE=134 CR3_BEFORE=83443712 fs_base=0 gs_base=18446744071564586576
[23427822330] [INFO] [sprout::pipelines] [CPU2] SPROUT: Compositor ready
[23430571560] [INFO] [bloom] [CPU3] bloom: [bloom] SIMD backend: SSE2 (x86_64)
[23431596705] [INFO] [echo] [CPU1] echo: starting up
[23435081307] [INFO] [echo] [CPU1] echo: ready for Bristle events (main loop using handle 14)
[23447341500] [INFO] [bloom] [CPU3] [bloom] EARLY boot args: bristle_evt=12 arg_req=1 arg_resp=4
[23450819832] [INFO] [bloom::compositor] [CPU3] bloom: map_from_bytespace start bs=1233
[23466687948] [INFO] [bloom::compositor] [CPU3] bloom: symbols loaded
[23469881886] [INFO] [bloom::compositor] [CPU3] bloom: loop iter 1
[23470987386] [INFO] [virtio::device] [CPU2] VirtIO: mapping common BAR4...
[23472680385] [INFO] [kernel::syscall::handlers::device] [CPU2] DEVICE: Mapped BAR4 phys=0xc000004000 size=0x4000 -> virt=0x107f5000
[23474698302] [INFO] [virtio::device] [CPU2] VirtIO: common_cfg at 0x107f5000
[23476110801] [INFO] [virtio::device] [CPU2] VirtIO: notify_cfg at 0x107f8000
[23479143171] [INFO] [virtio::device] [CPU2] VirtIO: device_cfg = Some(276787200)
[23480865012] [INFO] [virtio::device] [CPU2] VirtIO: allocating DMA command buffer...
[23482881015] [INFO] [bloom::compositor] [CPU3] bloom: got width 1920
[23485104093] [INFO] [virtio::device] [CPU2] VirtIO: cmd_buf virt=0x107f9000 phys=0x5035000
[23487450129] [INFO] [virtio::device] [CPU2] VirtIO: device::new complete
[23490163488] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[23491375446] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Device features 0x30bf8024
[23492597106] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[23494109199] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: MAC 52:54:00:12:34:56
[23495642478] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Link UP
[23507255904] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX queue 0 setup (size=32)
[23509915374] [INFO] [bloom::compositor] [CPU3] bloom: got properties w=1920 h=1080 s=7680 f=1
[23512423572] [INFO] [bloom::compositor] [CPU3] bloom: properties OK, building from config...
[23514930252] [INFO] [bloom::compositor] [CPU3] bloom: compositor bytespace 1233 (1920x1080 stride=7680 format=1)
[23518761222] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX queue 1 setup (size=32)
[23530069860] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Allocated RX pool (16 pages, 32 buffers)
[23531552253] [INFO] [bloom::compositor] [CPU3] bloom: display backend: VirtIO-GPU
[23533915053] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Allocated TX buffer (1 page)
[23536779783] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX queue filled with 32 buffers
[23538277521] [INFO] [bloom::compositor] [CPU3] bloom: mapped size=8294400 (source=bytespace_info)
[23539817631] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: DRIVER_OK set, device is live
[23541979791] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: Driver initialized successfully
[23544048759] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Driver initialized successfully
[23545521549] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: MAC 52:54:00:12:34:56
[23546778354] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Waiting for link...
[23548136733] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Link is UP
[23554002417] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created port (write=23, read=24)
[23559591759] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created RX port (write=25, read=26)
[23567914854] [INFO] [bloom::present] [CPU3] bloom: driver REGISTER (kind=2 caps=0x3)
[23570395794] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 4] Proof of life (Beeper)
[23572177134] [INFO] [bloom::present] [CPU3] display: negotiated proto 1.0 caps=DIRTY_RECTS|FULLFRAME max_rects=32
[23573990055] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawning beeper...
[23575577421] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/beeper
[23576341965] [INFO] [display_virtio_gpu] [CPU1] display_virtio_gpu: received MSG_HELLO
[23577338268] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[23579122215] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[23582757759] [INFO] [bloom] [CPU3] [bloom] Requesting initial driver buffer...
[23585394690] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=205000 exec=false
[23587370994] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=207000 exec=false
[23591698284] [INFO] [bloom] [CPU3] [bloom] ACQUIRED RETURNED!
[23595870870] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 29 (user task/process) assigned to CPU 0
[23599639008] [INFO] [sprout::pipelines] [CPU2] SPROUT: Spawned beeper (PID=29)
[23601326463] [INFO] [sprout::supervisor] [CPU2] SPROUT: [Stage 5] Starting Discovered User Apps
[23602782324] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/clock'
[23604422556] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/clock
[23605164264] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[23607461031] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[23615803035] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=209000 exec=false
[23618114751] [INFO] [bloom] [CPU3] [bloom] ACQUIRED initial driver buffer: 0x10ffc000 (bs_id=ThingId([214, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]))
[23620935690] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=20e000 exec=false
[23621859096] [INFO] [bloom] [CPU3] bloom: creating surface...
[23623029606] [INFO] [bloom] [CPU3] bloom: surface created!
[23623995615] [INFO] [bloom] [CPU3] bloom: finding UI CROWN...
[23629784970] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 30 (user task/process) assigned to CPU 3
[23632580829] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=30)
[23634026625] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/hello_std'
[23635241553] [INFO] [bloom] [CPU3] bloom: creating new UI CROWN
[23636272638] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/hello_std
[23637576831] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[23639299266] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[23642501751] [DEBUG] [kernel::sched::spawn] [CPU3] Trampoline entered. Arg: 0xffffffffb01111e0
[23643851220] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=202000 exec=false
[23648353014] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=203000 exec=false
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[23650461219] [INFO] [bran::arch::x86_64::enter_user] [CPU3] Entering user mode tid=30 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370703888 RFLAGS_BEFORE=130 CR3_BEFORE=84361216 fs_base=0 gs_base=18446744071564586640
[23667512583] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 31 (user task/process) assigned to CPU 1
[23669841921] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([235, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[23672684607] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=31)
[23674058034] [INFO] [sprout::supervisor] [CPU2] SPROUT: Launching discovered app '/boot/telnetd'
[23675163204] [INFO] [stem::ui] [CPU3] UiBuilder: created root 1260
[23676216102] [INFO] [kernel::task::loader] [CPU2] Loading module: /boot/telnetd
[23677160430] [INFO] [bloom] [CPU3] bloom: UI CROWN initialized!
[23677928670] [DEBUG] [kernel::task::loader] [CPU2]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[23679916557] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=200000 exec=true
[23683365255] [DEBUG] [kernel::sched::spawn] [CPU1] Trampoline entered. Arg: 0xffffffffb00f2278
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[23686011789] [INFO] [bran::arch::x86_64::enter_user] [CPU1] Entering user mode tid=31 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370769424 RFLAGS_BEFORE=130 CR3_BEFORE=84512768 fs_base=0 gs_base=18446744071564586576
[23694632478] [ERROR] [kernel::trap] [CPU1] user_invalid_opcode rip=0x0000000000201420 err=0x0000 rsp=0x00000000007fff08 cs=0x2b kind=6
[23703243762] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 32 (user thread) assigned to CPU 3
[23707051632] [INFO] [bloom] [CPU3] bloom: spawned asset watcher (tid=32)
[23708148222] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=220000 exec=false
[23709169605] [INFO] [bloom::frame_loop] [CPU3] bloom: running (fps_target=60)
[23710877388] [INFO] [bloom] [CPU3] [bloom] bristle_evt_handle = 12 (legacy was 12)
T:0AF0 [23714883786] [INFO] [kernel::task::loader] [CPU2] Segment: vaddr=228000 exec=false
[23716156233] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] asset_watcher_entry: spawning sub-loaders
[23733548751] [DEBUG] [kernel::sched::spawn] [CPU2] SCHED: Task 33 (user task/process) assigned to CPU 2
[23737781364] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 34 (user thread) assigned to CPU 3
[23739439581] [INFO] [sprout::supervisor] [CPU2] SPROUT: App launched (PID=33)
[23741539932] [INFO] [sprout::supervisor] [CPU2] SPROUT: Startup complete. Entering monitor loop.
[23748095943] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 35 (user thread) assigned to CPU 3
[23756635881] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 36 (user thread) assigned to CPU 3
[23758568790] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Created service node ThingId([235, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
T:1050 T:0EC0 T:F930 [23765217267] [DEBUG] [kernel::sched::spawn] [CPU2] Trampoline entered. Arg: 0xffffffffb00f2298
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[23766865518] [INFO] [bran::arch::x86_64::enter_user] [CPU2] Entering user mode tid=33 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562324605 RSP_BEFORE=18446744072370901264 RFLAGS_BEFORE=130 CR3_BEFORE=84615168 fs_base=0 gs_base=18446744071564586608
[23773639560] [INFO] [telnetd] [CPU2] telnetd: starting on port 2323
[23809902138] [INFO] [telnetd] [CPU2] telnetd: waiting for network stack
[23858046762] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] wallpaper loader: loading flower.bmp
[23864798034] [INFO] [flytrap] [CPU2] FLYTRAP: Created Limine ContentSource node
[23866660455] [INFO] [flytrap] [CPU2] FLYTRAP: Performing initial boot module scan...
[23868182547] [DEBUG] [kernel::sched::spawn] [CPU3] SCHED: Task 37 (user thread) assigned to CPU 3
[23870861355] [DEBUG] [bloom::asset] [CPU3] [asset_bank] worker spawned tid=37 (priority=2)
[23873896728] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] cursor loader: loading default cursor
[23876924313] [INFO] [taskman] [CPU2] TASKMAN: Found UI Crown (attempt 3)
[23878102281] [DEBUG] [bloom::painter_resources] [CPU3] [bloom] icon loader started
[23883790524] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Published service - TX port=23, RX port=26
T:1AA0 [23890436625] [DEBUG] [bloom::asset] [CPU3] [asset_bank] worker started (priority bump)
[23892854667] [DEBUG] [bloom::asset] [CPU3] [asset_bank] load_wallpaper_immediate: flower.bmp
[23901118263] [INFO] [fetchd] [CPU3] FETCHD: Found UI Root: 1260
[23904384306] [INFO] [netd] [CPU3] NETD: Found driver service node ThingId([235, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[23915341857] [INFO] [bloom] [CPU3] bloom: GPU composition not supported (or Virgl disabled) - using CPU composition mode
[23917907277] [INFO] [bloom] [CPU3] bloom: calling intern for FONT_GLYPH
[23937273624] [INFO] [bloom] [CPU3] bloom: intern returned FONT_GLYPH=265
[23938897917] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for FONT_GLYPH
[23940434793] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[23941912500] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[23943552402] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[23945315130] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=265 subj_lo=0
[23962207830] [INFO] [anther::net_client] [CPU1] anther: Network stack not found
[23963502882] [INFO] [anther] [CPU1] anther: Waiting for network stack...
[23964255414] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1269)
[23965384113] [INFO] [bloom] [CPU3] bloom: calling intern for UI_WINDOW
[23978903685] [INFO] [bloom] [CPU3] bloom: intern returned UI_WINDOW=242
[23980359117] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for UI_WINDOW
[23981375715] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[23982779766] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[23984116431] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[23985487515] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=242 pred=0 subj_lo=0
[23998947984] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1271)
[24000713055] [INFO] [bloom] [CPU3] bloom: calling intern for UI_PAINT_GEN
[24006139113] [INFO] [telnetd] [CPU2] telnetd: waiting for network stack
[24011903520] [INFO] [netd] [CPU3] NETD: Driver TX port=23, RX port=26, link_up=true, mtu=1500
[24015499959] [INFO] [netd] [CPU3] NETD: Connected to driver - MAC 52:54:00:12:34:56
[24020354193] [INFO] [netd] [CPU3] NETD: Created socket API port (write=27, read=28)
[24025922778] [INFO] [bloom] [CPU3] bloom: intern returned UI_PAINT_GEN=267
[24027469785] [INFO] [bloom] [CPU3] bloom: calling root_watch_open for UI_PAINT_GEN
[24028558686] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x7f5570
[24029550204] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[24030624915] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[24031944849] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=267 subj_lo=0
[24047350074] [INFO] [bloom] [CPU3] bloom: root_watch_open ret=Some(1276)
[24049769205] [INFO] [bloom] [CPU3] [bloom] Starting UI loop immediately (not waiting for fonts)
[24107885208] [INFO] [netd] [CPU3] NETD: Published initial stack node ThingId([251, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) to graph
[24126533475] [INFO] [telnetd::net_client] [CPU2] telnetd: connected to socket API (netd=27, write=29, read=30)
[24192254196] [INFO] [netd] [CPU3] NETD: Starting DHCP...
[24193387020] [INFO] [anther::net_client] [CPU1] anther: Connected to netd socket API (netd_port=27, our_write=31, our_read=32)
[24195288282] [INFO] [netd::dhcp] [CPU3] DHCP: Starting discovery...
[24197864724] [INFO] [anther] [CPU1] anther: Connected to network stack
[24203983518] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 304 bytes - IPv4 other
[24243235236] [INFO] [netd::dhcp] [CPU3] DHCP: Deconfigured
[24515584632] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping bytespace 741 (8386614 bytes) for 'flower.bmp'
[24549024258] [DEBUG] [bloom::asset] [CPU3] [asset_bank] decoding BMP for 'flower.bmp'...
[24611697957] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=4000 nodes=642 watches=13 history=1024 journal=1024 symbols=294 drops=0
[24973462074] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 304 bytes
[25013406660] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/sprout' (raw, 111120 bytes, hash=77c80b059e863710)
[25544740023] [INFO] [anther::net_client] [CPU1] anther: TCP_LISTEN timeout
[25546738437] [INFO] [anther] [CPU1] anther: Failed to listen on port 80, retrying...
[25630136202] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=0 len=600
[25659774855] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 590 byte frame (594 encoded) to netd rx_port=25
[25663349283] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (594 bytes sent)
[25728817950] [DEBUG] [bloom::asset] [CPU3] [asset_bank] BMP decoded: 2048x1365 for 'flower.bmp'
[26248106676] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=5000 nodes=668 watches=13 history=1024 journal=1024 symbols=332 drops=0
[26945156535] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 590 bytes
[26961403293] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_wallpaper (pending): 2048x1365
[26963468268] [DEBUG] [bloom::asset] [CPU3] [asset_bank] load_cursor_immediate: /assets/cursors/future/default.svg
[26982374298] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 316 bytes - IPv4 other
[27133876803] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[27135190665] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[27136659000] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[27137990649] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=333 pred=0 subj_lo=0
[27253794513] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[27395004681] [INFO] [fetchd] [CPU3] FETCHD: Entering main loop, watching for network stack...
[27410040240] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[27411788646] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[27413443728] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[27415194477] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=334 pred=0 subj_lo=0
[27530608347] [INFO] [bloom::present] [CPU3] display: full-frame damage, using full-frame present
[27595099092] [INFO] [bloom] [CPU3] [CONTRACT] [bloom] First frame rendered
[27606299457] [DEBUG] [bloom::reclaimer] [CPU3] [reclaimer] +11182080 bytes (total: 11182080)
[27608401887] [INFO] [bloom::asset] [CPU3] [asset_bank] promoting wallpaper 'flower.bmp' to gen=1 (11182080b)
[27622645314] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping bytespace 791 (3051 bytes)
[27679823886] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapped to 0x12004000
[27681659907] [DEBUG] [bloom::asset] [CPU3] [asset_bank] detected SVG format
[27683498337] [DEBUG] [bloom::asset] [CPU3] [asset_bank] parsing SVG cursor from /assets/cursors/future/default.svg
[27774746868] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=27 flat_sq=0.003125 edges=36
[27788603040] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=10 flat_sq=0.003125 edges=19
[27794418300] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=60 flat_sq=0.003125 edges=45
[27797280060] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=10 flat_sq=0.003125 edges=32
[27800241480] [INFO] [bloom::raster] [CPU3] [raster] build_edges: verbs=95 flat_sq=0.003125 edges=72
[27805106571] [DEBUG] [bloom::asset] [CPU3] [asset_bank] SUCCESS: SVG cursor rasterized 64x64
[27816873546] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_cursor (pending)
[27818558097] [DEBUG] [bloom::asset] [CPU3] [asset_bank] cursor: Static frame 64x64 hotspot (12, 8)
[27820885950] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapping font bytespace 771 (569208 bytes)
[27836224086] [DEBUG] [bloom::asset] [CPU3] [asset_bank] mapped at 0x12005000
[27837867387] [DEBUG] [bloom::asset] [CPU3] [asset_bank] parsing font '/assets/fonts/NotoSans-Regular.ttf'...
[28029694935] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=6000 nodes=709 watches=15 history=1024 journal=1024 symbols=349 drops=0
[28040675388] [INFO] [anther::net_client] [CPU1] anther: TCP_LISTEN timeout
[28041946647] [INFO] [anther] [CPU1] anther: Failed to listen on port 80, retrying...
[28046299512] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[28047679077] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[28049019570] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[28050484077] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=335 pred=0 subj_lo=0
[28138328196] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 316 bytes
[28666984830] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=1 len=600
[28669743069] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 590 byte frame (594 encoded) to netd rx_port=25
[28672401912] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (594 bytes sent)
[28942835229] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 590 bytes
[28947132522] [INFO] [netd::dhcp] [CPU3] DHCP: Configuration received
[28949581386] [INFO] [netd] [CPU3] NETD: DHCP complete - IP: 10.0.2.15, Gateway: 10.0.2.2, DNS: 10.0.2.3
[29564481144] [INFO] [anther::net_client] [CPU1] anther: TCP_LISTEN timeout
[29565820911] [INFO] [anther] [CPU1] anther: Failed to listen on port 80, retrying...
[29735894034] [INFO] [netd] [CPU3] NETD: DHCP configured ��� IP: 10.0.2.15, GW: 10.0.2.2, DNS: 10.0.2.3
[29738191659] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=7000 nodes=722 watches=16 history=1024 journal=1024 symbols=356 drops=0
[29739838062] [INFO] [netd] [CPU3] NETD: Network stack ready, entering service loop
[29744494890] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_LISTEN on port 2323 with backlog 64
[29748381531] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Listening on port 2323, handle=1
[29750941407] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_LISTEN on port 80 with backlog 64
[29752228836] [INFO] [telnetd] [CPU2] telnetd: listening on guest port 2323 (handle=1)
[29753677734] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Listening on port 80, handle=2
[29755573914] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_LISTEN on port 80 with backlog 64
[29757751650] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Listening on port 80, handle=3
[29760030234] [INFO] [netd::socket_api] [CPU3] SOCKET_API: UDP_BIND on port 5353
[29762812761] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Bound UDP on port 5353, handle=4
[29765461341] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_LISTEN on port 80 with backlog 64
[29767348413] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Listening on port 80, handle=5
[29800823514] [INFO] [fetchd] [CPU3] FETCHD: Got IP address: 10.0.2.15
[29815536102] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Joining multicast group 224.0.0.251
[29818709316] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 42 bytes - IPv4 other
[29826471114] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 42 bytes
[29943436347] [DEBUG] [bloom::asset] [CPU3] [asset_bank] SUCCESS: font parsed
[29965074810] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=2 len=70
[29966500542] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[29968823181] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[29974045959] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[29975361471] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=3 len=70
[29979307842] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[29982297048] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[29984198145] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[29989113660] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 42 bytes - ARP
[29994257139] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 42 bytes
[29995143915] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_LISTEN on port 80 with backlog 64
[29996724450] [INFO] [netd::socket_api] [CPU3] SOCKET_API: Listening on port 80, handle=6
[29998668150] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=4 len=74
[30000298152] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 64 byte frame (68 encoded) to netd rx_port=25
[30004384542] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (68 bytes sent)
[30016817193] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 64 bytes
[30021489465] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 58 bytes - TCP SYN-ACK
[30024687792] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 58 bytes - TCP SYN-ACK
[30041635833] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[30043899006] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 58 bytes
[30047103768] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/bristle' (raw, 41456 bytes, hash=8aec727fb8928c70)
[30062777415] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: ptr=0x400000040870
[30063987921] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: validating range len=48
[30065243010] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: copyin success. mode=1 start_seq=0
[30066588156] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU3] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=336 pred=0 subj_lo=0
[30080268471] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=5 len=70
[30081982821] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[30085845867] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[30097960860] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[30106828455] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=6 len=70
[30107916993] [INFO] [anther] [CPU1] anther: Listening on port 80 (handle=6)
[30109932237] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[30112025559] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[30113465712] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[30117128712] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 54 bytes - TCP ACK
[30125462334] [DEBUG] [bloom::asset] [CPU3] [asset_bank] publish_font (pending): 'NotoSans-Regular.ttf' in slot 0
[30154601136] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 54 bytes
[31835566125] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=8000 nodes=759 watches=17 history=1024 journal=1024 symbols=361 drops=0
[32340975414] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 58 bytes - TCP SYN-ACK
[32347085100] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 58 bytes
[32368288854] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=7 len=70
[32370326307] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[32373384912] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[32773564461] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[32782614711] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[32788621866] [INFO] [netd::socket_api] [CPU3] SOCKET_API: TCP_ACCEPT handle=1 socket state=Established
[32791600842] [DEBUG] [netd::socket_api] [CPU3] SOCKET_API: Connection established from 10.0.2.2:42142 on listener 1
[34052132499] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=9000 nodes=768 watches=17 history=1024 journal=1024 symbols=361 drops=0
[34114650801] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/rtc_cmos' (raw, 29248 bytes, hash=562b9fa5b03c7a45)
[35845205088] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 60 bytes - TCP ACK
[35961858537] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 60 bytes
[36036696564] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=8 len=85
[36038756754] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 75 byte frame (79 encoded) to netd rx_port=25
[36041199843] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (79 bytes sent)
[36077527761] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 75 bytes
[36083012295] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 72 bytes - TCP ACK
[36142141794] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=10000 nodes=782 watches=17 history=1024 journal=1024 symbols=361 drops=0
[36194264106] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 72 bytes
[37895160600] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 74 bytes - TCP ACK
[37972685388] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 98 bytes - TCP ACK
[38179288950] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=9 len=70
[38213841072] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[38226069354] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[38228895606] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 74 bytes
[38440946907] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=10 len=70
[38444908821] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[38447890602] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[38586268974] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=11000 nodes=796 watches=17 history=1024 journal=1024 symbols=361 drops=0
[40163729892] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[40165398999] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[40169693256] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 105 bytes - TCP ACK
[40530049461] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=12000 nodes=802 watches=17 history=1024 journal=1024 symbols=361 drops=0
[40616568102] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 105 bytes
[42761764782] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=11 len=70
[42764049900] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[42766903641] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[42805495755] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[42807456021] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 55 bytes - TCP ACK
[42905666628] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=13000 nodes=806 watches=17 history=1024 journal=1024 symbols=361 drops=0
[43191358914] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 55 bytes
[44281031685] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=12 len=70
[44283223116] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[44285681517] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[44526972930] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[44529056418] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 57 bytes - TCP ACK
[44643829065] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 57 bytes
[44903638956] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=14000 nodes=816 watches=17 history=1024 journal=1024 symbols=361 drops=0
[45402699012] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=13 len=70
[45404507478] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[45406996833] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[45811990983] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[45813928710] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 58 bytes - TCP ACK
[45966735309] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[46023906159] [INFO] [phloem::executor] [CPU2] phloem: entering discover_nodes
[46025834151] [INFO] [phloem::executor] [CPU2] phloem: starting BFS discovery from roots
[46030021983] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 58 bytes
[46048714668] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=14 len=70
[46051993317] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[46056444291] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[46066781574] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[46069103421] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 60 bytes - TCP ACK
[46074795063] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 60 bytes
[46088399940] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=15 len=70
[46090748781] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[46093447125] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[46123140492] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/clock' (raw, 61968 bytes, hash=cae77ff7f85d3f2f)
[46180348302] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[46235404314] [DEBUG] [bloom::reclaimer] [CPU3] [reclaimer] +16384 bytes (total: 11198464)
[46237882218] [INFO] [bloom::asset] [CPU3] [asset_bank] promoting cursor to gen=2 (16384b)
[46241218749] [DEBUG] [bloom::reclaimer] [CPU3] [reclaimer] +102400 bytes (total: 11300864)
[46243813374] [INFO] [bloom::asset] [CPU3] [asset_bank] promoting font 'NotoSans-Regular.ttf' to gen=2 (102400b) in slot 0
[46390568499] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[46464920403] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/taskman' (raw, 70160 bytes, hash=d3dcbbde76a4f136)
[46700607690] [INFO] [phloem::executor] [CPU2] phloem: BFS seeded with 530 nodes
[46715963250] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/ps2_kbd' (raw, 25072 bytes, hash=671285c0c9ba41ed)
[46782591570] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=15000 nodes=840 watches=17 history=1024 journal=1024 symbols=374 drops=0
[46978367370] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/echo' (raw, 33264 bytes, hash=91427bae3069f281)
[47342813958] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/bloom' (raw, 969224 bytes, hash=bf04a3a0a7e7228b)
[47629756647] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/ps2_mouse' (raw, 29168 bytes, hash=4e1c9cf03988a560)
[47981378181] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/display_bootfb' (raw, 29248 bytes, hash=fd9c527f4c1617be)
[48289402425] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/display_virtio_gpu' (raw, 53864 bytes, hash=bc0074690049f9cd)
[48423893496] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[48591483501] [INFO] [taskman] [CPU2] TASKMAN: Window created, found 1 initial tasks
[48599991792] [WARN] [WARN] [CPU2] root_watch_open: WATCH_START_LATEST is deprecated; use start_seq=0 + watch_drain.
[48602375151] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[48604138506] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[48605899749] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=18446744073709551615
[48607624758] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=226 subj_lo=0
[48627813366] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: ptr=0x7fefb0
[48628885107] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: validating range len=48
[48629957475] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: copyin success. mode=1 start_seq=18446744073709551615
[48631422939] [DEBUG] [kernel::syscall::handlers::root_handlers] [CPU2] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=366 pred=0 subj_lo=0
[48654534984] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/fontd' (raw, 188944 bytes, hash=55b938b69928e656)
[48951591546] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/blossom' (raw, 123408 bytes, hash=99095735adb015b8)
[48968997957] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=16000 nodes=901 watches=19 history=1024 journal=1024 symbols=376 drops=0
[49005591096] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=2429 ops=1 watches=19
[49254119904] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/flytrap' (raw, 291352 bytes, hash=bf23e61b1474078c)
[49493003010] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/virtio_netd' (raw, 49680 bytes, hash=e044c79e0fe2b998)
[49748487492] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/rtl8168d' (raw, 45584 bytes, hash=a6a9e038341d5a28)
[50155692279] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/netd' (raw, 147984 bytes, hash=0e57f834b3afbcb1)
[50509787988] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/fetchd' (raw, 41488 bytes, hash=4e0f74e1bbfcc0ce)
[50925960327] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/anther' (raw, 586272 bytes, hash=c1e9a7b90a789e5c)
[51070243554] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=17000 nodes=937 watches=19 history=1024 journal=1024 symbols=377 drops=0
[51328353285] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/fortune' (raw, 61968 bytes, hash=543c2140ff247374)
[51763446603] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/photosynthesis' (raw, 12648 bytes, hash=a0b92df72a4fb68a)
[52197389409] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/ahci_disk' (raw, 45664 bytes, hash=89d4c04e59e9cee2)
[52413709194] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[52522875438] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[52618025262] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/iso9660d' (raw, 53856 bytes, hash=e0b08365f733f5f1)
[52630422669] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[52769434476] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[53034849648] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/virtio_sound' (raw, 49648 bytes, hash=cc70fc86a6ccc18f)
[53421737358] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=18000 nodes=986 watches=19 history=1024 journal=1024 symbols=377 drops=0
[53477547981] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/hdaudio' (raw, 37360 bytes, hash=efe87e268a526308)
[53971976190] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/pci_stubd' (raw, 29168 bytes, hash=cd275cc95b190f74)
[54445785207] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/beeper' (raw, 33296 bytes, hash=3662850ee0db2f79)
[54879021516] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/nectar' (raw, 176656 bytes, hash=9e2e57ba5e34944a)
[55215689430] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[55302315519] [INFO] [phloem::executor] [CPU2] phloem: discovered 550 nodes
[55318655403] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 66 bytes - TCP ACK
[55575772824] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 66 bytes
[55610593830] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=19000 nodes=1027 watches=19 history=1024 journal=1024 symbols=377 drops=0
[55933857804] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=16 len=70
[55936431408] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[55939833015] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[57366663948] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[57374213127] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 56 bytes - TCP ACK
[57633381399] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=20000 nodes=1033 watches=19 history=1024 journal=1024 symbols=377 drops=0
[57817679535] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 56 bytes
[57823833507] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/hello_std' (raw, 16880 bytes, hash=bcb44329b85efc54)
[58080485595] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=17 len=70
[58083233340] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[58086669894] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[59018193333] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[59024092215] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 55 bytes - TCP ACK
[59529720987] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=21000 nodes=1041 watches=19 history=1024 journal=1024 symbols=377 drops=0
[60060794739] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 55 bytes
[61116137115] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=18 len=70
[61117503546] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[61119702468] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[61205613051] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=22000 nodes=1051 watches=19 history=1024 journal=1024 symbols=377 drops=0
[61251911358] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[61253833476] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 66 bytes - TCP ACK
[61279483683] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[61395508317] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[62007892320] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 66 bytes
[62075687223] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=19 len=70
[62077224132] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[62079387051] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[62645331441] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[62647260654] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 107 bytes - TCP ACK
[63165351480] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 107 bytes
[63310951935] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=23000 nodes=1069 watches=19 history=1024 journal=1024 symbols=377 drops=0
[63560824833] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=20 len=70
[63562976598] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[63566855913] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[65187975831] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=24000 nodes=1077 watches=19 history=1024 journal=1024 symbols=377 drops=0
[65953094427] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[65959091880] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 65 bytes - TCP ACK
[65974368207] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 65 bytes
[66302771766] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=21 len=70
[66306494133] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[66312596328] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[66735807864] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[66785402277] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[66799043553] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 63 bytes - TCP ACK
[67028162124] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 63 bytes
[68121260658] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[68340036138] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[68358258210] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=25000 nodes=1097 watches=19 history=1024 journal=1024 symbols=377 drops=0
[68680042266] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=22 len=70
[68689219533] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[68693507553] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[68755631010] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[68758480692] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 133 bytes - TCP ACK
[68975160870] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 133 bytes
[69804886734] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[70294331118] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=23 len=70
[70297204164] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[70300842117] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[70341794622] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[70362758829] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 87 bytes - TCP ACK
[70440416970] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=26000 nodes=1111 watches=19 history=1024 journal=1024 symbols=377 drops=0
[70449595887] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[70465438857] [INFO] [bloom::cursor_rasterizer] [CPU3] [cursor_rasterizer] rasterizing cursor snapshot gen=2
[70576277475] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[70666892637] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[70772303316] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[70891641249] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[71086175094] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 87 bytes
[71154131202] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=24 len=70
[71157507564] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[71160222474] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[71329949724] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[71332966122] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 197 bytes - TCP ACK
[71944925415] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 197 bytes
[72807935145] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=27000 nodes=1129 watches=19 history=1024 journal=1024 symbols=377 drops=0
[73430722035] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=25 len=70
[73433229474] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[73436568777] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[74166162213] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[74167858182] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 76 bytes - TCP ACK
[74189282937] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[74222986926] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 76 bytes
[74354884791] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=26 len=70
[74357649333] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[74360905773] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[74405772375] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[74408364690] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 65 bytes - TCP ACK
[74666751390] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=28000 nodes=1143 watches=19 history=1024 journal=1024 symbols=377 drops=0
[74816959833] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 65 bytes
[74916238056] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=27 len=70
[74918352333] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[74920841358] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[75401983833] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[75405455400] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 75 bytes - TCP ACK
[75516108129] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[75613163538] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[75807740448] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 75 bytes
[76080443076] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[76467256503] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=29000 nodes=1162 watches=19 history=1024 journal=1024 symbols=377 drops=0
[77479662150] [INFO] [kernel::root::handlers::batch] [CPU1] ROOT COMMIT: seq=3453 ops=1 watches=19
[77978206416] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=30000 nodes=1168 watches=19 history=1024 journal=1024 symbols=377 drops=0
[78646140375] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=28 len=70
[78648265476] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[78651111429] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[78663912921] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[78665770722] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 164 bytes - TCP ACK
[78910711440] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 164 bytes
[79608774234] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=31000 nodes=1174 watches=19 history=1024 journal=1024 symbols=377 drops=0
[79681082316] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[79704185946] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=29 len=70
[79706669460] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[79710338070] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[80896390641] [INFO] [flytrap] [CPU2] FLYTRAP: Published new asset '/boot/stdio_demo' (raw, 16880 bytes, hash=c2c5f3a4bb2c1649)
[81103948398] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[81106918332] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 66 bytes - TCP ACK
[81716802849] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 66 bytes
[82232095044] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=32000 nodes=1184 watches=19 history=1024 journal=1024 symbols=377 drops=0
[82329560874] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[83080613055] [INFO] [fontd] [CPU3] FONTD: Processed 1 ASSET events (0 fonts)
[83202484794] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: RX frame! desc=30 len=70
[83204803902] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Forwarding 60 byte frame (64 encoded) to netd rx_port=25
[83207679753] [INFO] [virtio_netd] [CPU2] VIRTIO_NETD: Frame forwarded successfully (64 bytes sent)
[83314398849] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: RX frame from driver, 60 bytes
[83317357167] [INFO] [netd::ipc_device] [CPU3] IpcNicDevice: TX 126 bytes - TCP ACK
[83334027876] [INFO] [virtio_netd::driver] [CPU2] VirtIO-NET: TX 126 bytes
[83883702309] [INFO] [kernel::root::service] [CPU1] ROOT STATS: iter=33000 nodes=1190 watches=19 history=1024 journal=1024 symbols=377 drops=0
[85182979233] [INFO]
```
</details>
