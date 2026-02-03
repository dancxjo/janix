# ✅ Scenario: Core services and drivers are launched by the supervisor

> Last run: 2026-02-03 21:10:42

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booting | ✅ | 343ms | <a href="./01/after.png"><img src="./01/after.png" width="150" /></a> - [💾](./01/registers.txt) |
| 2 | When I wait for the system to reach ready state | ✅ | 7482ms | <a href="./02/after.png"><img src="./02/after.png" width="150" /></a> [📜](./02/serial.log) [💾](./02/registers.txt) |
| 3 | Then the serial output should contain "SPROUT: Supervisor starting..." | ✅ | 2367ms | <a href="./03/after.png"><img src="./03/after.png" width="150" /></a> [📜](./03/serial.log) [💾](./03/registers.txt) |
| 4 | And the serial output should contain "SPROUT: Launching app '/boot/ingestd'" | ✅ | 1345ms | <a href="./04/after.png"><img src="./04/after.png" width="150" /></a> [📜](./04/serial.log) [💾](./04/registers.txt) |
| 5 | And the serial output should contain "SPROUT: Launching app '/boot/fontd'" | ✅ | 1375ms | <a href="./05/after.png"><img src="./05/after.png" width="150" /></a> - [💾](./05/registers.txt) |
| 6 | And the serial output should contain "SPROUT: Launching app '/boot/blossom'" | ✅ | 1356ms | <a href="./06/after.png"><img src="./06/after.png" width="150" /></a> - [💾](./06/registers.txt) |
| 7 | And the serial output should contain "SPROUT: Launching app '/boot/cambium'" | ✅ | 208ms | - - - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[14981198543] [INFO] [kernel] BOOTFB: width=1920 height=1080 pitch=7680 bpp=32 format=Bgrx8888
[15019177261] [CONTRACT] [kernel] thing-os kernel starting...
[15065428622] [INFO] [kernel::memory] Memory map has 51 entries
[15090035114] [INFO] [kernel::memory]   [0] 0x0 - 0xa0000 (Usable)
[15117361009] [INFO] [kernel::memory]   [1] 0x100000 - 0x800000 (Usable)
[15145708372] [INFO] [kernel::memory]   [2] 0x800000 - 0x808000 (Other)
[15184310146] [INFO] [kernel::memory]   [3] 0x808000 - 0x80b000 (Usable)
[15214990683] [INFO] [kernel::memory]   [4] 0x80b000 - 0x80c000 (Other)
[15246203818] [INFO] [kernel::memory]   [5] 0x80c000 - 0x811000 (Usable)
[15276437610] [INFO] [kernel::memory]   [6] 0x811000 - 0x900000 (Other)
[15307169383] [INFO] [kernel::memory]   [7] 0x900000 - 0x1780000 (Reserved)
[15338109099] [INFO] [kernel::memory]   [8] 0x1780000 - 0x78e0e000 (Usable)
[15370454265] [INFO] [kernel::memory]   [9] 0x78e0e000 - 0x78e6a000 (Reserved)
[15403422444] [INFO] [kernel::memory]   [10] 0x78e6a000 - 0x78e6b000 (Other)
[15438321487] [INFO] [kernel::memory]   [11] 0x78e6b000 - 0x78e6c000 (Reserved)
[15476041035] [INFO] [kernel::memory]   [12] 0x78e6c000 - 0x78e6d000 (Other)
[15513700318] [INFO] [kernel::memory]   [13] 0x78e6d000 - 0x78e6e000 (Reserved)
[15546814686] [INFO] [kernel::memory]   [14] 0x78e6e000 - 0x78ef9000 (Other)
[15580513062] [INFO] [kernel::memory]   [15] 0x78ef9000 - 0x7901a000 (Other)
[15613426544] [INFO] [kernel::memory]   [16] 0x7901a000 - 0x7904c000 (Other)
[15646253365] [INFO] [kernel::memory]   [17] 0x7904c000 - 0x7905c000 (Other)
[15680038129] [INFO] [kernel::memory]   [18] 0x7905c000 - 0x79079000 (Other)
[15712446668] [INFO] [kernel::memory]   [19] 0x79079000 - 0x79086000 (Other)
[15744788897] [INFO] [kernel::memory]   [20] 0x79086000 - 0x79280000 (Other)
[15777395915] [INFO] [kernel::memory]   [21] 0x79280000 - 0x7a16c000 (Reserved)
[15810904878] [INFO] [kernel::memory]   [22] 0x7a16c000 - 0x7bb6c000 (Usable)
[15845284222] [INFO] [kernel::memory]   [23] 0x7bb6c000 - 0x7bb90000 (Reserved)
[15879332207] [INFO] [kernel::memory]   [24] 0x7bb90000 - 0x7bb97000 (Other)
[15913498884] [INFO] [kernel::memory]   [25] 0x7bb97000 - 0x7bb9e000 (Other)
[15946541642] [INFO] [kernel::memory]   [26] 0x7bb9e000 - 0x7bba5000 (Other)
[15979354085] [INFO] [kernel::memory]   [27] 0x7bba5000 - 0x7bbae000 (Other)
[16011805136] [INFO] [kernel::memory]   [28] 0x7bbae000 - 0x7bbca000 (Other)
[16044736457] [INFO] [kernel::memory]   [29] 0x7bbca000 - 0x7bbd2000 (Other)
[16077282129] [INFO] [kernel::memory]   [30] 0x7bbd2000 - 0x7bbea000 (Other)
[16110036263] [INFO] [kernel::memory]   [31] 0x7bbea000 - 0x7bc0a000 (Reserved)
[16143403164] [INFO] [kernel::memory]   [32] 0x7bc0a000 - 0x7e1fb000 (Usable)
[16177642616] [INFO] [kernel::memory]   [33] 0x7e1fb000 - 0x7e93f000 (Reserved)
[16211466145] [INFO] [kernel::memory]   [34] 0x7e93f000 - 0x7ea00000 (Reserved)
[16247385632] [INFO] [kernel::memory]   [35] 0x7ea00000 - 0x7f4ed000 (Reserved)
[16282100404] [INFO] [kernel::memory]   [36] 0x7f4ed000 - 0x7f5ed000 (Reserved)
[16317376162] [INFO] [kernel::memory]   [37] 0x7f5ed000 - 0x7f6ed000 (Reserved)
[16353494794] [INFO] [kernel::memory]   [38] 0x7f6ed000 - 0x7f76d000 (Reserved)
[16388385072] [INFO] [kernel::memory]   [39] 0x7f76d000 - 0x7f77f000 (Acpi)
[16423317667] [INFO] [kernel::memory]   [40] 0x7f77f000 - 0x7f7ff000 (Other)
[16457665380] [INFO] [kernel::memory]   [41] 0x7f7ff000 - 0x7fec1000 (Reserved)
[16491210167] [INFO] [kernel::memory]   [42] 0x7fec1000 - 0x7fec5000 (Reserved)
[16525556058] [INFO] [kernel::memory]   [43] 0x7fec5000 - 0x7fec7000 (Other)
[16560347396] [INFO] [kernel::memory]   [44] 0x7fec7000 - 0x7fef4000 (Reserved)
[16593516060] [INFO] [kernel::memory]   [45] 0x7fef4000 - 0x7ff78000 (Reserved)
[16629345069] [INFO] [kernel::memory]   [46] 0x7ff78000 - 0x80000000 (Other)
[16665791808] [INFO] [kernel::memory]   [47] 0x80000000 - 0x807e9000 (Framebuffer)
[16699995087] [INFO] [kernel::memory]   [48] 0xe0000000 - 0xf0000000 (Reserved)
[16738107794] [INFO] [kernel::memory]   [49] 0xffc00000 - 0x100000000 (Reserved)
[16772948129] [INFO] [kernel::memory]   [50] 0xfd00000000 - 0x10000000000 (Reserved)
[16809788415] [INFO] [kernel::memory] HHDM Offset: 0xffff800000000000
[17148547348] [CONTRACT] [kernel::memory] Frame allocator initialized with 507415 free frames
[17189701844] [INFO] [bran::arch] IOAPIC: hhdm=0xffff800000000000
[17254498896] [INFO] [bran::arch] IOAPIC: Disabling legacy PIC...
[17282526840] [INFO] [bran::arch] IOAPIC: PIC disabled OK
[17308947617] [INFO] [bran::arch] IOAPIC: RSDP virt=0x7f77e014
[17341551617] [INFO] [bran::arch] IOAPIC: MADT parsed OK
[17367260669] [INFO] [bran::arch] IOAPIC: Found at phys 0xfec00000, GSI base 0
[17395982787] [INFO] [bran::arch] IOAPIC: Registers initialized
[17429438219] [INFO] [bran::arch] IOAPIC: version 0x20, 24 redir entries
[17457746712] [INFO] [bran::arch] IOAPIC: All pins masked
[17488608426] [INFO] [bran::arch] IOAPIC: IRQ1 -> GSI 1 -> 0x21
[17514202369] [INFO] [bran::arch] IOAPIC: IRQ12 -> GSI 12 -> 0x2C
[17541162767] [INFO] [bran::arch] IOAPIC: Init complete
[17567113855] [CONTRACT] [kernel] Initializing global allocator...
[18355829399] [INFO] [kernel::memory::global_alloc] Global allocator initialized (LinkedHeap, 32MB)
[18398047584] [CONTRACT] [kernel] Initializing SIMD...
[18441374685] [CONTRACT] [kernel] Initializing tasking...
[18491060924] [INFO] [kernel::task::scheduler]   Acquiring scheduler lock...
[18517112248] [INFO] [kernel::task::scheduler]   Lock acquired, checking if initialized...
[18551941988] [INFO] [kernel::task::scheduler]   Allocating scheduler...
[18598932686] [INFO] [kernel::task::scheduler]   Leaking scheduler...
[18630796184] [INFO] [kernel::task::scheduler]   Initializing boot task...
[18661637131] [INFO] [kernel::task::scheduler]   Creating boot task...
[18699154883] [INFO] [kernel::task::scheduler]   Creating idle task...
[18737292062] [INFO] [kernel::task::scheduler]   Creating graph worker task...
[18772086906] [INFO] [kernel::task::scheduler]   Boot task initialized
[18806498423] [INFO] [kernel::task::scheduler]   Storing scheduler pointer...
[18838089378] [CONTRACT] [kernel::task::scheduler] Scheduler initialized
[18879312680] [INFO] [kernel::root] Spawning Root service...
[18912075849] [INFO] [kernel::root::boot_register] ROOT: boot registration begin (Census Phase 1 v0.2)
[18991104983] [INFO] [kernel::root::service] ROOT: started once
[19489404940] [CONTRACT] [alloc] MEDIUM ALLOC #1: 120 KB align=8 total=0MB
[19852965971] [INFO] [kernel::root::service] ROOT STATS: iter=500 nodes=84 watches=0 history=487 journal=411 symbols=56
[19933817640] [CONTRACT] [alloc] MEDIUM ALLOC #2: 240 KB align=8 total=0MB
[20146198640] [INFO] [kernel::root::boot_register] ROOT: Census Phase 2: PCI
[20177504004] [INFO] [kernel::root::pci] PCI: Starting enumeration...
[20273253378] [INFO] [kernel::root::pci] PCI: 00:00.0 8086:29c0 Intel Corporation 82G33/G31/P35/P31 Express DRAM Controller class=06:00 prog_if=00 rev=00
[20342102044] [INFO] [kernel::root::pci] PCI: 00:01.0 1234:1111 (unknown vendor) (unknown device) class=03:00 prog_if=00 rev=02
[20427466713] [INFO] [kernel::root::pci] PCI:   BAR0: phys=0x80000000 size=0x1000000
[20485790122] [INFO] [kernel::root::pci] PCI:   BAR2: phys=0x810c5000 size=0x1000
[20556187008] [INFO] [kernel::root::pci] PCI: 00:02.0 8086:10d3 Intel Corporation 82574L Gigabit Network Connection class=02:00 prog_if=00 rev=00
[20613739314] [INFO] [kernel::root::pci] PCI:   BAR0: phys=0x810a0000 size=0x20000
[20676757072] [INFO] [kernel::root::pci] PCI:   BAR1: phys=0x81080000 size=0x20000
[20713490369] [INFO] [kernel::root::pci] PCI:   BAR3: phys=0x810c0000 size=0x4000
[20790834805] [INFO] [kernel::root::pci] PCI: 00:1f.0 8086:2918 Intel Corporation 82801IB (ICH9) LPC Interface Controller class=06:01 prog_if=00 rev=02
[20839930282] [INFO] [kernel::root::pci] PCI: Found LPC/ISA bridge at 00:1f.0
[20959170467] [INFO] [kernel::root::pci] LPC: Created Legacy IO bus with CMOS and PS/2 controller
[21028004068] [INFO] [kernel::root::pci] PCI: 00:1f.2 8086:2922 Intel Corporation 82801IR/IO/IH (ICH9R/DO/DH) 6 port SATA Controller [AHCI mode] class=01:06 prog_if=01 rev=02
[21089772850] [INFO] [kernel::root::pci] PCI:   BAR5: phys=0x810c4000 size=0x1000
[21167380285] [INFO] [kernel::root::pci] PCI: Found AHCI SATA controller at 00:1f.2
[21211956592] [INFO] [kernel::root::pci] PCI: Registered AHCI controller (graph_id=119, idx=3) BAR5=0x810c4000
[21288730653] [INFO] [kernel::root::pci] PCI: 00:1f.3 8086:2930 Intel Corporation 82801I (ICH9 Family) SMBus Controller class=0c:05 prog_if=00 rev=02
[21350513765] [INFO] [kernel::root::boot_register] ROOT: registered items. host=1 kernel=3
[21417712476] [CONTRACT] [kernel] KERNEL: root census complete: host=t1 kernel=t3 root=t4
[21460749022] [INFO] [kernel] Found init module: /boot/sprout (cmdline: 'init'), loading...
[21504111194] [INFO] [kernel::task::loader] Loading module: /boot/sprout
[21543351610] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21597777576] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[21668643021] [INFO] [kernel::task::loader] Segment: vaddr=211000 exec=false
[21705501046] [INFO] [kernel::task::loader] Segment: vaddr=216000 exec=false
[21753136587] [INFO] [kernel] Spawning sprout with registry at 0x600000...
[21787580033] [CONTRACT] [kernel] Spawning init process...
[21822079256] [INFO] [kernel] System initialized. Setting up preemption timer (100Hz)...
[21872923679] [INFO] [bran::arch::x86_64::ioapic] LAPIC: calibrated timer (61941200 ticks/sec), init_cnt=619412 for 100Hz
[21924347374] [CONTRACT] [kernel] Entering scheduler loop.
[21984527592] [INFO] [kernel::tests::time_test] TIME ANCHORING TEST: Starting...
[22014813277] [INFO] [kernel::tests::time_test] TIME ANCHORING TEST: Success - sys_time_now returned EAGAIN before anchor
[22064317262] [INFO] [kernel::tests::time_test] TIME ANCHORING TEST: Anchoring clock...
[22130228381] [INFO] [kernel::time] System clock anchored: unix_secs=1706126400, mono_ns=1000, offset=1706126399999999000ns
[22184606250] [INFO] [kernel::tests::time_test] TIME ANCHORING TEST: Success - sys_time_now returned valid time after anchor: 1706126411092012971
[22257934528] [INFO] [kernel::tests::time_test] TIME ANCHORING TEST: PASS - All invariants verified
[22334636399] [INFO] [kernel::tests::time_monotonic_test] TIME MONOTONIC TEST: Starting...
[22392228543] [INFO] [kernel::tests::time_monotonic_test] TIME MONOTONIC TEST: 1000 samples monotonic - PASS
[22444976274] [INFO] [kernel::tests::time_monotonic_test] TIME MONOTONIC TEST: trace::now() monotonic - PASS
[22505075914] [INFO] [kernel::tests::time_monotonic_test] TIME MONOTONIC TEST: trace and syscall consistent - PASS
[22565897982] [INFO] [kernel::tests::time_monotonic_test] TIME MONOTONIC TEST: All tests PASS
[22625984665] [INFO] [kernel::tests::fw_tables] FW_TABLES SELFTEST: Starting...
[22673624028] [INFO] [kernel::tests::fw_tables] FW_TABLES SELFTEST: PASS
[22715275017] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0000178
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600000
[22809600848] [INFO] [task.user_enter] Entering user mode tid=4 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562188157 RSP_BEFORE=18446744072367660960 RFLAGS_BEFORE=134 CR3_BEFORE=50331648 fs_base=0 gs_base=18446744071564038400
[22881471685] [INFO] [sprout] [sprout] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ff9b0 rip=0x2013ff rflags=0x202
[23015460319] [INFO] [sprout] SPROUT: v0.4 starting (Supervisor Mode)...
[23067382915] [INFO] [sprout::devtree] SPROUT: devtree::init entry (v0.2)
[23103713420] [INFO] [sprout::devtree] SPROUT: Step 1: Find Host
[23148083914] [INFO] [sprout::devtree] SPROUT: Step 2: HHDM
[23183389384] [INFO] [sprout::devtree] SPROUT: Step 3: Platform Bus
[23214483691] [INFO] [sprout::devtree] SPROUT: Step 4: Firmware
[23247033272] [INFO] [sprout::devtree] SPROUT: Finding ACPI...
[23277437220] [INFO] [sprout::devtree] SPROUT: Found 1 ACPI nodes
[23320542539] [INFO] [sprout::devtree] SPROUT: ACPI RSDP = 0x7f77e014
[23351574750] [INFO] [sprout::devtree] SPROUT: Finding DTB...
[23387757597] [INFO] [sprout::devtree] SPROUT: Found 0 DTB nodes
[23415803154] [INFO] [sprout::devtree] SPROUT: Init OK, returning context
[23448708521] [INFO] [sprout::devtree] SPROUT: build() called
[23481253984] [INFO] [sprout::devtree::x86_64] SPROUT: x86_64 platform enrichment... (v0.2)
[23517256049] [INFO] [sprout::devtree::x86_64] SPROUT: x86_64 enumerate done
[23562900826] [INFO] [sprout] SPROUT: About to create Supervisor...
[23597068909] [INFO] [sprout] SPROUT: Supervisor created, calling run_forever...
[23632529441] [INFO] [sprout::supervisor] SPROUT: Supervisor starting...
[23670646062] [INFO] [sprout::supervisor] SPROUT: Discovering modules...
[23709686408] [INFO] [sprout::supervisor] SPROUT: Found 15 modules
[23826745571] [INFO] [sprout::supervisor] SPROUT: Module[0] = '/boot/sprout'
[23866508841] [INFO] [sprout::supervisor] SPROUT: Module[1] = '/boot/bristle'
[23907805201] [INFO] [sprout::supervisor] SPROUT: Module[2] = '/boot/root_canal'
[23949908998] [INFO] [sprout::supervisor] SPROUT: Module[3] = '/boot/rtc_cmos'
[23991931813] [INFO] [sprout::supervisor] SPROUT: Module[4] = '/boot/ps2_kbd'
[24037282449] [INFO] [sprout::supervisor] SPROUT: Module[5] = '/boot/echo'
[24081453454] [INFO] [sprout::supervisor] SPROUT: Module[6] = '/boot/ps2_mouse'
[24124034336] [INFO] [sprout::supervisor] SPROUT: Module[7] = '/boot/virtio_netd'
[24169361341] [INFO] [sprout::supervisor] SPROUT: Module[8] = '/boot/netd'
[24213791010] [INFO] [sprout::supervisor] SPROUT: Module[9] = '/boot/fetchd'
[24254645067] [INFO] [sprout::supervisor] SPROUT: Module[10] = '/boot/pollen'
[24296925913] [INFO] [sprout::supervisor] SPROUT: Module[11] = '/assets/wallpapers/leather.bmp'
[24341302780] [INFO] [sprout::supervisor] SPROUT: Module[12] = '/assets/fonts/NotoSans-Regular.ttf'
[24395019712] [INFO] [sprout::supervisor] SPROUT: Module[13] = '/assets/cursors/future/default.svg'
[24452596761] [INFO] [sprout::supervisor] SPROUT: Module[14] = '/boot/locale.conf'
[24503451446] [INFO] [sprout::registry] SPROUT: Scanning boot modules...
[24561646355] [INFO] [sprout::registry] SPROUT: Registering driver 'dev.rtc.Cmos' -> '/boot/rtc_cmos' (fallback)
[24636520873] [INFO] [sprout::registry] SPROUT: Registry scan complete. Found 1 drivers.
[24693094217] [INFO] [sprout::supervisor] SPROUT: spawn_apps start. tasks len=0
[24739572728] [INFO] [sprout::supervisor] SPROUT: Adding fallback app '/boot/ingestd'
[24781531754] [INFO] [sprout::supervisor] SPROUT: Adding fallback app '/boot/fontd'
[24825214899] [INFO] [sprout::supervisor] SPROUT: Adding fallback app '/boot/blossom'
[24869012819] [INFO] [sprout::supervisor] SPROUT: Adding fallback app '/boot/cambium'
[24912354074] [INFO] [sprout::supervisor] SPROUT: Adding fallback app '/boot/ahci_disk'
[24956276298] [INFO] [sprout::supervisor] SPROUT: Adding fallback app '/boot/iso_reader'
[25000600139] [INFO] [sprout::supervisor] SPROUT: Adding fallback app '/boot/font_explorer'
[25045119606] [INFO] [sprout::supervisor] SPROUT: Adding fallback app '/boot/photosynthesis'
[25091996105] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/ingestd'
[25139736974] [INFO] [sprout::supervisor] SPROUT: Failed to launch app '/boot/ingestd': ENOENT
[25181839026] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/fontd'
[25228869976] [INFO] [sprout::supervisor] SPROUT: Failed to launch app '/boot/fontd': ENOENT
[25269142598] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/blossom'
[25318998018] [INFO] [sprout::supervisor] SPROUT: Failed to launch app '/boot/blossom': ENOENT
[25360135297] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/cambium'
[25410121454] [INFO] [sprout::supervisor] SPROUT: Failed to launch app '/boot/cambium': ENOENT
[25452141624] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/ahci_disk'
[25498226316] [INFO] [sprout::supervisor] SPROUT: Failed to launch app '/boot/ahci_disk': ENOENT
[25540627860] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/iso_reader'
[25591464351] [INFO] [sprout::supervisor] SPROUT: Failed to launch app '/boot/iso_reader': ENOENT
[25634465872] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/font_explorer'
[25682539339] [INFO] [sprout::supervisor] SPROUT: Failed to launch app '/boot/font_explorer': ENOENT
[25728722298] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/photosynthesis'
[25781067625] [INFO] [sprout::supervisor] SPROUT: Failed to launch app '/boot/photosynthesis': ENOENT
[25829124224] [INFO] [sprout::pipelines] SPROUT: Setting up display pipeline...
[25890291382] [INFO] [sprout::pipelines] SPROUT: Using boot framebuffer (fallback)
[25931449086] [INFO] [sprout::pipelines] SPROUT: Display backend: BootFB (1920x1080 stride=7680)
[26596925035] [INFO] [sprout::supervisor] SPROUT: Found match for RTC: driver '/boot/rtc_cmos'
[26648173748] [INFO] [kernel::task::loader] Loading module: /boot/rtc_cmos
[26693104611] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[26738552328] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[26804229804] [INFO] [kernel::task::loader] Segment: vaddr=205000 exec=false
[26844800796] [INFO] [kernel::task::loader] Segment: vaddr=207000 exec=false
[26925106697] [INFO] [sprout::supervisor] SPROUT: Driver launched (PID=5)
[26969298378] [INFO] [sprout::pipelines] SPROUT: Setting up input pipeline (keyboard + mouse)...
[27010267161] [INFO] [sprout::pipelines] SPROUT: Created kbd_raw port (w=5, r=6)
[27072294506] [INFO] [sprout::pipelines] SPROUT: Created mouse_raw port (w=7, r=8)
[27126333060] [INFO] [sprout::pipelines] SPROUT: Created evt port (w=9, r=10)
[27169087601] [INFO] [kernel::task::loader] Loading module: /boot/ps2_kbd
[27205575902] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[27249798785] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[27314531095] [INFO] [kernel::task::loader] Segment: vaddr=204000 exec=false
[27354254113] [INFO] [kernel::task::loader] Segment: vaddr=205000 exec=false
[27403018421] [INFO] [sprout::pipelines] SPROUT: Spawned ps2_kbd (PID=6)
[27442126479] [INFO] [kernel::task::loader] Loading module: /boot/ps2_mouse
[27476794905] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[27522054080] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[27586841321] [INFO] [kernel::task::loader] Segment: vaddr=204000 exec=false
[27627919614] [INFO] [kernel::task::loader] Segment: vaddr=205000 exec=false
[27677339857] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0000178
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x73
[27805859211] [INFO] [task.user_enter] Entering user mode tid=5 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=0 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562188157 RSP_BEFORE=18446744072367689040 RFLAGS_BEFORE=134 CR3_BEFORE=59076608 fs_base=0 gs_base=18446744071564038400
[27860827131] [INFO] [rtc_cmos] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ffeb0 rip=0x201027 rflags=0x202
[27990050101] [INFO] [rtc_cmos] Starting... arg=73
[28036244829] [INFO] [rtc_cmos] Serving device ID: ThingId([115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[28069079476] [INFO] [rtc_cmos] RTC: 2026-02-03 21:11:00 = 1770153060 unix_secs
[28125343524] [INFO] [kernel::time] System clock anchored: unix_secs=1770153060, mono_ns=14062542414, offset=1770153045937457586ns
[28174157596] [INFO] [rtc_cmos] System clock anchored
[28241196102] [INFO] [rtc_cmos] RTC: Set sys.TimeState = 1 (Anchored)
[28280373351] [INFO] [rtc_cmos] Publishing time. Entering maintenance loop.
[28316729232] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0031f48
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x5
[28405453647] [INFO] [task.user_enter] Entering user mode tid=6 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562188157 RSP_BEFORE=18446744072367722496 RFLAGS_BEFORE=134 CR3_BEFORE=59195392 fs_base=0 gs_base=18446744071564038400
[28465518419] [INFO] [ps2_kbd] ps2_kbd: online (handle=5)
[28593516536] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0038050
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x7
[28673415700] [INFO] [task.user_enter] Entering user mode tid=7 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562188157 RSP_BEFORE=18446744072367739168 RFLAGS_BEFORE=130 CR3_BEFORE=59305984 fs_base=0 gs_base=18446744071564038400
[28726087147] [INFO] [ps2_mouse] ps2_mouse: online (handle=7)
[28851178351] [INFO] [ps2_mouse] ps2_mouse: enabling aux port
[28879655934] [INFO] [sprout::pipelines] SPROUT: Spawned ps2_mouse (PID=7)
[28911890008] [INFO] [sprout::pipelines] SPROUT: Created evt_echo port (w=11, r=12)
[28952802597] [INFO] [kernel::task::loader] Loading module: /boot/bristle
[28993435663] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[29037636865] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[29105696492] [INFO] [kernel::task::loader] Segment: vaddr=204000 exec=false
[29147039189] [INFO] [kernel::task::loader] Segment: vaddr=206000 exec=false
[29196657329] [INFO] [sprout::pipelines] SPROUT: Spawned bristle (PID=8)
[29240016769] [INFO] [ps2_kbd] ps2_kbd: created driver node 127
[29275165550] [INFO] [kernel::syscall::handlers::device] DEVICE: task subscribed to vector 0x21
[29312355553] [INFO] [ps2_kbd] ps2_kbd: subscribed to IRQ1 (vector 0x21)
[29360766163] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0000178
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600080009000b
[29449187240] [INFO] [task.user_enter] Entering user mode tid=8 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562188157 RSP_BEFORE=18446744072367766192 RFLAGS_BEFORE=130 CR3_BEFORE=59416576 fs_base=0 gs_base=18446744071564038400
[29504659029] [INFO] [bristle] bristle: online (kbd=6, mouse=8, evt=9, evt_echo=11)
[29646984940] [INFO] [ps2_mouse] ps2_mouse: controller cfg already correct (0x47)
[29688447609] [INFO] [ps2_mouse] ps2_mouse: sending enable command (0xF4)
[29731535478] [INFO] [ps2_mouse] ps2_mouse: enable ACK received (0xFA)
[29773371610] [INFO] [bristle] bristle: registered in graph as svc.Input (id=130)
[29822985035] [INFO] [ps2_kbd] ps2_kbd: entering interrupt-driven loop
[29871477320] [INFO] [sprout::pipelines] SPROUT: Bloom handles via BS=128 backend=BootFB
[29911629393] [ERROR] [sprout::pipelines] SPROUT: Failed to spawn bloom: ENOENT
[29959223235] [INFO] [kernel::task::loader] Loading module: /boot/echo
[29997972588] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[30041528237] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[30106461534] [INFO] [kernel::task::loader] Segment: vaddr=204000 exec=false
[30145996616] [INFO] [kernel::task::loader] Segment: vaddr=205000 exec=false
[30195590398] [INFO] [sprout::pipelines] SPROUT: Spawned echo (PID=9)
[30232265869] [INFO] [sprout::pipelines] SPROUT: Input pipeline ready (keyboard + mouse)
[30270808527] [INFO] [sprout::pipelines] SPROUT: Setting up network pipeline...
[30316454597] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0000178
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xc
[30439353824] [INFO] [task.user_enter] Entering user mode tid=9 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=0 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562188157 RSP_BEFORE=18446744072368043696 RFLAGS_BEFORE=130 CR3_BEFORE=59539456 fs_base=0 gs_base=18446744071564038400
[30493415151] [INFO] [echo] echo: online (handle=12)
[30615277053] [INFO] [echo] echo: ready for Bristle events (keyboard + mouse)
[30650233634] [INFO] [sprout::pipelines] SPROUT: No NIC device found, skipping network pipeline
[30690446855] [INFO] [sprout::supervisor] SPROUT: Entering supervisor loop.
[30945914122] [INFO] [ps2_mouse] ps2_mouse: init done
[30978818790] [INFO] [kernel::syscall::handlers::device] DEVICE: task subscribed to vector 0x2c
[31013832048] [INFO] [ps2_mouse] ps2_mouse: subscribed to IRQ12 (vector 0x2c)
[31061857558] [INFO] [ps2_mouse] ps2_mouse: entering interrupt-driven loop

```
</details>
