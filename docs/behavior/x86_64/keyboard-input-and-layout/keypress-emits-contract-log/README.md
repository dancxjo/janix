# ❌ Scenario: Keypress emits contract log

> Last run: 2026-01-20 17:49:56

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the clock window is ticking | ✅ | 9532ms | <a href="./01/after.png"><img src="./01/after.png" width="150" /></a> - [💾](./01/registers.txt) |
| 2 | When I press a key | ❌ | 1012ms | - [📜](./02/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[10051458153] [CONTRACT] [kernel] thing-os kernel starting...
[10060540677] [INFO] [kernel::memory] Memory map has 64 entries
[10062471639] [INFO] [kernel::memory]   [0] 0x0 - 0xa0000 (Usable)
[10063098078] [INFO] [kernel::memory]   [1] 0x100000 - 0x800000 (Usable)
[10063743063] [INFO] [kernel::memory]   [2] 0x800000 - 0x808000 (Other)
[10064136918] [INFO] [kernel::memory]   [3] 0x808000 - 0x80b000 (Usable)
[10064456424] [INFO] [kernel::memory]   [4] 0x80b000 - 0x80c000 (Other)
[10064766360] [INFO] [kernel::memory]   [5] 0x80c000 - 0x811000 (Usable)
[10065081180] [INFO] [kernel::memory]   [6] 0x811000 - 0x900000 (Other)
[10065390654] [INFO] [kernel::memory]   [7] 0x900000 - 0x1780000 (Reserved)
[10065736923] [INFO] [kernel::memory]   [8] 0x1780000 - 0x78659000 (Usable)
[10066064613] [INFO] [kernel::memory]   [9] 0x78659000 - 0x786bb000 (Reserved)
[10066401246] [INFO] [kernel::memory]   [10] 0x786bb000 - 0x7883d000 (Other)
[10066748769] [INFO] [kernel::memory]   [11] 0x7883d000 - 0x7883e000 (Reserved)
[10067091375] [INFO] [kernel::memory]   [12] 0x7883e000 - 0x788d5000 (Other)
[10067422299] [INFO] [kernel::memory]   [13] 0x788d5000 - 0x788d6000 (Reserved)
[10067769624] [INFO] [kernel::memory]   [14] 0x788d6000 - 0x78977000 (Other)
[10068150741] [INFO] [kernel::memory]   [15] 0x78977000 - 0x78978000 (Reserved)
[10068496944] [INFO] [kernel::memory]   [16] 0x78978000 - 0x789b8000 (Other)
[10068828165] [INFO] [kernel::memory]   [17] 0x789b8000 - 0x789b9000 (Reserved)
[10069171398] [INFO] [kernel::memory]   [18] 0x789b9000 - 0x78a44000 (Other)
[10069503642] [INFO] [kernel::memory]   [19] 0x78a44000 - 0x78a45000 (Reserved)
[10069862979] [INFO] [kernel::memory]   [20] 0x78a45000 - 0x78a91000 (Other)
[10070193969] [INFO] [kernel::memory]   [21] 0x78a91000 - 0x78a92000 (Reserved)
[10072539444] [INFO] [kernel::memory]   [22] 0x78a92000 - 0x78a98000 (Other)
[10072871655] [INFO] [kernel::memory]   [23] 0x78a98000 - 0x78a99000 (Reserved)
[10073229837] [INFO] [kernel::memory]   [24] 0x78a99000 - 0x78f1a000 (Other)
[10073562477] [INFO] [kernel::memory]   [25] 0x78f1a000 - 0x78f1b000 (Reserved)
[10073906733] [INFO] [kernel::memory]   [26] 0x78f1b000 - 0x7939c000 (Other)
[10074239505] [INFO] [kernel::memory]   [27] 0x7939c000 - 0x7939d000 (Reserved)
[10074583035] [INFO] [kernel::memory]   [28] 0x7939d000 - 0x7969e000 (Other)
[10074914784] [INFO] [kernel::memory]   [29] 0x7969e000 - 0x7969f000 (Reserved)
[10075257456] [INFO] [kernel::memory]   [30] 0x7969f000 - 0x796a8000 (Other)
[10075589172] [INFO] [kernel::memory]   [31] 0x796a8000 - 0x796a9000 (Reserved)
[10075934385] [INFO] [kernel::memory]   [32] 0x796a9000 - 0x796ad000 (Other)
[10076271480] [INFO] [kernel::memory]   [33] 0x796ad000 - 0x796ae000 (Reserved)
[10076632599] [INFO] [kernel::memory]   [34] 0x796ae000 - 0x796b0000 (Other)
[10076965371] [INFO] [kernel::memory]   [35] 0x796b0000 - 0x796b1000 (Reserved)
[10077310782] [INFO] [kernel::memory]   [36] 0x796b1000 - 0x796b3000 (Other)
[10077717870] [INFO] [kernel::memory]   [37] 0x796b3000 - 0x796b4000 (Reserved)
[10078071696] [INFO] [kernel::memory]   [38] 0x796b4000 - 0x796b6000 (Other)
[10078448556] [INFO] [kernel::memory]   [39] 0x796b6000 - 0x796b7000 (Reserved)
[10078994673] [INFO] [kernel::memory]   [40] 0x796b7000 - 0x796b9000 (Other)
[10079510628] [INFO] [kernel::memory]   [41] 0x796b9000 - 0x796ba000 (Reserved)
[10079887356] [INFO] [kernel::memory]   [42] 0x796ba000 - 0x796bc000 (Other)
[10080221976] [INFO] [kernel::memory]   [43] 0x796bc000 - 0x796bd000 (Reserved)
[10080568377] [INFO] [kernel::memory]   [44] 0x796bd000 - 0x796c7000 (Other)
[10080899169] [INFO] [kernel::memory]   [45] 0x796c7000 - 0x796c8000 (Reserved)
[10081241214] [INFO] [kernel::memory]   [46] 0x796c8000 - 0x796cc000 (Other)
[10081573590] [INFO] [kernel::memory]   [47] 0x796cc000 - 0x796cd000 (Reserved)
[10081915074] [INFO] [kernel::memory]   [48] 0x796cd000 - 0x796cf000 (Other)
[10082247153] [INFO] [kernel::memory]   [49] 0x796cf000 - 0x796d0000 (Reserved)
[10082591706] [INFO] [kernel::memory]   [50] 0x796d0000 - 0x796d2000 (Other)
[10082923422] [INFO] [kernel::memory]   [51] 0x796d2000 - 0x796d3000 (Reserved)
[10083282396] [INFO] [kernel::memory]   [52] 0x796d3000 - 0x796d7000 (Other)
[10083614739] [INFO] [kernel::memory]   [53] 0x796d7000 - 0x79750000 (Other)
[10083946323] [INFO] [kernel::memory]   [54] 0x79750000 - 0x79907000 (Other)
[10084277511] [INFO] [kernel::memory]   [55] 0x79907000 - 0x7a16c000 (Reserved)
[10084621470] [INFO] [kernel::memory]   [56] 0x7a16c000 - 0x7bb6c000 (Usable)
[10084957542] [INFO] [kernel::memory]   [57] 0x7bb6c000 - 0x7bb8d000 (Reserved)
[10085299917] [INFO] [kernel::memory]   [58] 0x7bb8d000 - 0x7bb91000 (Other)
[10085631237] [INFO] [kernel::memory]   [59] 0x7bb91000 - 0x7bb92000 (Reserved)
[10085973876] [INFO] [kernel::memory]   [60] 0x7bb92000 - 0x7bb94000 (Other)
[10086322290] [INFO] [kernel::memory]   [61] 0x7bb94000 - 0x7bb95000 (Reserved)
[10086663675] [INFO] [kernel::memory]   [62] 0x7bb95000 - 0x7bb97000 (Other)
[10086995589] [INFO] [kernel::memory]   [63] 0x7bb97000 - 0x7bb98000 (Reserved)
[10087549263] [INFO] [kernel::memory] HHDM Offset: 0xffff800000000000
[10320244770] [CONTRACT] [kernel::memory] Frame allocator initialized with 495729 free frames
[10326554601] [INFO] [bran::arch] IOAPIC: hhdm=0xffff800000000000
[10330948947] [INFO] [bran::arch] IOAPIC: Disabling legacy PIC...
[10331970099] [INFO] [bran::arch] IOAPIC: PIC disabled OK
[10332626007] [INFO] [bran::arch] IOAPIC: RSDP virt=0x7f77e014
[10338723219] [INFO] [bran::arch] IOAPIC: MADT parsed OK
[10339165947] [INFO] [bran::arch] IOAPIC: Found at phys 0xfec00000, GSI base 0
[10341817959] [INFO] [bran::arch] IOAPIC: Registers initialized
[10342820697] [INFO] [bran::arch] IOAPIC: version 0x20, 24 redir entries
[10344266559] [INFO] [bran::arch] IOAPIC: All pins masked
[10345492014] [INFO] [bran::arch] IOAPIC: IRQ1 -> GSI 1 -> 0x21
[10346059350] [INFO] [bran::arch] IOAPIC: IRQ12 -> GSI 12 -> 0x2C
[10346522868] [INFO] [bran::arch] IOAPIC: Init complete
[10347115218] [CONTRACT] [kernel] Initializing global allocator...
[10670196207] [INFO] [kernel::memory::global_alloc] Global allocator initialized (LinkedHeap, 32MB)
[10670828322] [CONTRACT] [kernel] Initializing SIMD...
[10672096281] [CONTRACT] [kernel] Initializing tasking...
[10676565570] [INFO] [kernel::task::scheduler]   Acquiring scheduler lock...
[10678022520] [INFO] [kernel::task::scheduler]   Lock acquired, checking if initialized...
[10678574181] [INFO] [kernel::task::scheduler]   Allocating scheduler...
[10683920082] [INFO] [kernel::task::scheduler]   Leaking scheduler...
[10684312584] [INFO] [kernel::task::scheduler]   Initializing boot task...
[10685000634] [INFO] [kernel::task::scheduler]   Creating boot task...
[10689173385] [INFO] [kernel::task::scheduler]   Creating idle task...
[10693719168] [INFO] [kernel::task::scheduler]   Boot task initialized
[10694097381] [INFO] [kernel::task::scheduler]   Storing scheduler pointer...
[10694915055] [CONTRACT] [kernel::task::scheduler] Scheduler initialized
[10700285838] [INFO] [kernel::root] Spawning Root service...
[10707325761] [INFO] [kernel::root::boot_register] ROOT: boot registration begin (Census Phase 1 v0.2)
[10715661825] [INFO] [kernel::root::service] ROOT: started once
[11565994440] [INFO] [kernel::root::boot_register] ROOT: Census Phase 2: PCI
[11567117100] [INFO] [kernel::root::pci] PCI: Starting enumeration...
[11606442738] [INFO] [kernel::root::pci] PCI: 00:00.0 8086:29c0 Intel Corporation 82G33/G31/P35/P31 Express DRAM Controller class=06:00 prog_if=00 rev=00
[11623936599] [INFO] [kernel::root::pci] PCI: 00:01.0 1234:1111 (unknown vendor) (unknown device) class=03:00 prog_if=00 rev=02
[11649462132] [INFO] [kernel::root::pci] PCI: 00:02.0 8086:10d3 Intel Corporation 82574L Gigabit Network Connection class=02:00 prog_if=00 rev=00
[11679933474] [INFO] [kernel::root::pci] PCI: 00:1f.0 8086:2918 Intel Corporation 82801IB (ICH9) LPC Interface Controller class=06:01 prog_if=00 rev=02
[11682216546] [INFO] [kernel::root::pci] PCI: Found LPC/ISA bridge at 00:1f.0
[11717727351] [INFO] [kernel::root::pci] LPC: Created Legacy IO bus with CMOS and PS/2 controller
[11739795804] [INFO] [kernel::root::pci] PCI: 00:1f.2 8086:2922 Intel Corporation 82801IR/IO/IH (ICH9R/DO/DH) 6 port SATA Controller [AHCI mode] class=01:06 prog_if=01 rev=02
[11746708281] [INFO] [kernel::root::pci] PCI: Found AHCI SATA controller at 00:1f.2
[11749364946] [INFO] [kernel::root::pci] PCI: Registered AHCI controller (graph_id=224, idx=3) BAR5=0x810c4000
[11771365749] [INFO] [kernel::root::pci] PCI: 00:1f.3 8086:2930 Intel Corporation 82801I (ICH9 Family) SMBus Controller class=0c:05 prog_if=00 rev=02
[11773683801] [INFO] [kernel::root::boot_register] ROOT: registered items. host=3 kernel=6
[11774565759] [CONTRACT] [kernel] KERNEL: root census complete: host=t3 kernel=t6 root=t7
[11777634198] [INFO] [kernel] Found init module: /boot/sprout (cmdline: ''), loading...
[11779953504] [INFO] [kernel::task::loader] Loading module: /boot/sprout
[11780670594] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[11787621351] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[11798829273] [INFO] [kernel::task::loader] Segment: vaddr=20d230 exec=false
[11799656286] [INFO] [kernel::task::loader]   Overlap at 20d000: merging perms to r=true w=false x=true
[11801925036] [INFO] [kernel::task::loader] Segment: vaddr=2100a8 exec=false
[11802714924] [INFO] [kernel::task::loader]   Overlap at 210000: merging perms to r=true w=true x=true
[11810077488] [INFO] [kernel] Spawning sprout with registry at 0x600000...
[11810691783] [CONTRACT] [kernel] Spawning init process...
[11812142661] [INFO] [kernel] System initialized. Setting up preemption timer (100Hz)...
[11846630169] [INFO] [bran::arch::x86_64::ioapic] LAPIC: calibrated timer (62105800 ticks/sec), init_cnt=621058 for 100Hz
[11848036563] [CONTRACT] [kernel] Entering scheduler loop.
[11856636165] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb00084c8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600000
[11861128620] [INFO] [task.user_enter] Entering user mode tid=3 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132765 RSP_BEFORE=18446744072367676144 RFLAGS_BEFORE=134 CR3_BEFORE=50319360 fs_base=0 gs_base=18446744071563780584
[11870976051] [INFO] [sprout] [sprout] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ff9a0 rip=0x20038f rflags=0x206
[11872372281] [INFO] [sprout] SPROUT: v0.4 starting (Supervisor Mode)...
[11873110425] [INFO] [sprout::devtree] SPROUT: devtree::init entry (v0.2)
[11873803524] [INFO] [sprout::devtree] SPROUT: Step 1: Find Host
[11892509607] [INFO] [sprout::devtree] SPROUT: Step 2: HHDM
[11895910587] [INFO] [sprout::devtree] SPROUT: Step 3: Platform Bus
[11899092447] [INFO] [sprout::devtree] SPROUT: Step 4: Firmware
[11900027040] [INFO] [sprout::devtree] SPROUT: Finding ACPI...
[11903228568] [INFO] [sprout::devtree] SPROUT: Found 1 ACPI nodes
[11905427919] [INFO] [sprout::devtree] SPROUT: ACPI RSDP = 0x7f77e014
[11906148243] [INFO] [sprout::devtree] SPROUT: Finding DTB...
[11909360760] [INFO] [sprout::devtree] SPROUT: Found 0 DTB nodes
[11910051483] [INFO] [sprout::devtree] SPROUT: Init OK, returning context
[11910699867] [INFO] [sprout::devtree] SPROUT: build() called
[11911328748] [INFO] [sprout::devtree::x86_64] SPROUT: x86_64 platform enrichment... (v0.2)
[11915824800] [INFO] [sprout::devtree::x86_64] SPROUT: x86_64 enumerate done
[11916516909] [INFO] [sprout] SPROUT: About to create Supervisor...
[11917119258] [INFO] [sprout] SPROUT: Supervisor created, calling run_forever...
[11917899609] [INFO] [sprout::supervisor] SPROUT: Supervisor starting...
[11918553801] [INFO] [sprout::supervisor] SPROUT: Discovering modules...
[11923371867] [INFO] [sprout::supervisor] SPROUT: Found 42 modules
[11956716420] [INFO] [sprout::supervisor] SPROUT: Module[0] = '/boot/sprout'
[11961052092] [INFO] [sprout::supervisor] SPROUT: Module[1] = '/boot/bristle'
[11964149043] [INFO] [sprout::supervisor] SPROUT: Module[2] = '/boot/rtc_cmos'
[11967488577] [INFO] [sprout::supervisor] SPROUT: Module[3] = '/boot/clock'
[11969603778] [INFO] [sprout::supervisor] SPROUT: Discovered app: /boot/clock
[11972297832] [INFO] [sprout::supervisor] SPROUT: Module[4] = '/boot/font_explorer'
[11975544372] [INFO] [sprout::supervisor] SPROUT: Module[5] = '/boot/ps2_kbd'
[11978784609] [INFO] [sprout::supervisor] SPROUT: Module[6] = '/boot/echo'
[11981707353] [INFO] [sprout::supervisor] SPROUT: Module[7] = '/boot/bloom'
[11984698143] [INFO] [sprout::supervisor] SPROUT: Module[8] = '/boot/ps2_mouse'
[11987730645] [INFO] [sprout::supervisor] SPROUT: Module[9] = '/boot/root_batch_bench'
[11991711831] [INFO] [sprout::supervisor] SPROUT: Module[10] = '/boot/root_watch_tester'
[11995168779] [INFO] [sprout::supervisor] SPROUT: Module[11] = '/boot/display_bootfb'
[11997999024] [INFO] [sprout::supervisor] SPROUT: Module[12] = '/boot/ingestd'
[12000836958] [INFO] [sprout::supervisor] SPROUT: Module[13] = '/boot/cambium'
[12003600444] [INFO] [sprout::supervisor] SPROUT: Module[14] = '/boot/scheduler_fairness'
[12006307830] [INFO] [sprout::supervisor] SPROUT: Module[15] = '/boot/hogger'
[12009068544] [INFO] [sprout::supervisor] SPROUT: Module[16] = '/boot/tick_printer'
[12012671484] [INFO] [sprout::supervisor] SPROUT: Module[17] = '/assets/cursors/plain/Alternate.cur'
[12015823809] [INFO] [sprout::supervisor] SPROUT: Module[18] = '/assets/cursors/plain/Busy.cur'
[12018747906] [INFO] [sprout::supervisor] SPROUT: Module[19] = '/assets/cursors/plain/Diagonal1.ani'
[12021788295] [INFO] [sprout::supervisor] SPROUT: Module[20] = '/assets/cursors/plain/Diagonal2.ani'
[12024686421] [INFO] [sprout::supervisor] SPROUT: Module[21] = '/assets/cursors/plain/Handwriting.cur'
[12027540591] [INFO] [sprout::supervisor] SPROUT: Module[22] = '/assets/cursors/plain/Help.cur'
[12030536067] [INFO] [sprout::supervisor] SPROUT: Module[23] = '/assets/cursors/plain/Horizontal.ani'
[12034076736] [INFO] [sprout::supervisor] SPROUT: Module[24] = '/assets/cursors/plain/Link.ani'
[12037390662] [INFO] [sprout::supervisor] SPROUT: Module[25] = '/assets/cursors/plain/Move.cur'
[12040382244] [INFO] [sprout::supervisor] SPROUT: Module[26] = '/assets/cursors/plain/Normal.cur'
[12043607763] [INFO] [sprout::supervisor] SPROUT: Module[27] = '/assets/cursors/plain/Precision.cur'
[12046511598] [INFO] [sprout::supervisor] SPROUT: Module[28] = '/assets/cursors/plain/Text.cur'
[12049454538] [INFO] [sprout::supervisor] SPROUT: Module[29] = '/assets/cursors/plain/Unavailabe.cur'
[12052436121] [INFO] [sprout::supervisor] SPROUT: Module[30] = '/assets/cursors/plain/Vertical.ani'
[12055374177] [INFO] [sprout::supervisor] SPROUT: Module[31] = '/assets/cursors/plain/Working.ani'
[12058967019] [INFO] [sprout::supervisor] SPROUT: Module[32] = '/assets/wallpapers/clouds.bmp'
[12061840428] [INFO] [sprout::supervisor] SPROUT: Module[33] = '/assets/wallpapers/leather.bmp'
[12064861017] [INFO] [sprout::supervisor] SPROUT: Module[34] = '/assets/wallpapers/linen.bmp'
[12067974633] [INFO] [sprout::supervisor] SPROUT: Module[35] = '/assets/fonts/DSEG7Classic-Regular.ttf'
[12071017794] [INFO] [sprout::supervisor] SPROUT: Module[36] = '/assets/fonts/Hack-Regular.ttf'
[12073913907] [INFO] [sprout::supervisor] SPROUT: Module[37] = '/assets/fonts/NotoSans-Regular.ttf'
[12077071578] [INFO] [sprout::supervisor] SPROUT: Module[38] = '/assets/fonts/NotoSansSymbol-Regular.ttf'
[12080665344] [INFO] [sprout::supervisor] SPROUT: Module[39] = '/assets/fonts/NotoSansSymbol2-Regular.ttf'
[12083540964] [INFO] [sprout::supervisor] SPROUT: Module[40] = '/assets/fonts/NotoSerif-Regular.ttf'
[12086440344] [INFO] [sprout::supervisor] SPROUT: Module[41] = '/assets/pci/pci.ids'
[12088512513] [INFO] [sprout::registry] SPROUT: Scanning boot modules...
[12100065450] [INFO] [sprout::registry] SPROUT: Registering driver 'dev.rtc.Cmos' -> '/boot/rtc_cmos' (fallback)
[12163243026] [INFO] [sprout::registry] SPROUT: Registry scan complete. Found 1 drivers.
[12164076441] [INFO] [sprout::supervisor] SPROUT: spawn_apps start. tasks len=1
[12165849036] [INFO] [sprout::supervisor] SPROUT: Adding fallback app '/boot/font_explorer'
[12166795344] [INFO] [sprout::supervisor] SPROUT: Adding fallback app '/boot/ingestd'
[12167501577] [INFO] [sprout::supervisor] SPROUT: Adding fallback app '/boot/cambium'
[12168321561] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/clock'
[12170664528] [INFO] [kernel::task::loader] Loading module: /boot/clock
[12171185268] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[12172529127] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[12176135334] [INFO] [kernel::task::loader] Segment: vaddr=203e50 exec=false
[12176742204] [INFO] [kernel::task::loader]   Overlap at 203000: merging perms to r=true w=false x=true
[12178084545] [INFO] [kernel::task::loader] Segment: vaddr=204ee8 exec=false
[12178667358] [INFO] [kernel::task::loader]   Overlap at 204000: merging perms to r=true w=true x=true
[12233173590] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb00451d8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[12247552317] [INFO] [task.user_enter] Entering user mode tid=4 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=0 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132765 RSP_BEFORE=18446744072367986928 RFLAGS_BEFORE=134 CR3_BEFORE=50745344 fs_base=0 gs_base=18446744071563780584
[12252377709] [INFO] [clock] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ffea0 rip=0x2000cf rflags=0x206
[12253472550] [INFO] [clock] starting clock publisher
[12258587352] [INFO] [sprout::supervisor] SPROUT: App launched (PID=4)
[12261426441] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/font_explorer'
[12262402152] [INFO] [kernel::task::loader] Loading module: /boot/font_explorer
[12262927842] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[12263970246] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[12268217709] [INFO] [kernel::task::loader] Segment: vaddr=2041a0 exec=false
[12268797189] [INFO] [kernel::task::loader]   Overlap at 204000: merging perms to r=true w=false x=true
[12269612190] [INFO] [kernel::task::loader] Segment: vaddr=204668 exec=false
[12270197643] [INFO] [kernel::task::loader]   Overlap at 204000: merging perms to r=true w=true x=true
[12275683893] [INFO] [sprout::supervisor] SPROUT: App launched (PID=5)
[12276549945] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/ingestd'
[12277637823] [INFO] [kernel::task::loader] Loading module: /boot/ingestd
[12278138433] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[12279077283] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[12289029060] [INFO] [kernel::task::loader] Segment: vaddr=20f750 exec=false
[12289604976] [INFO] [kernel::task::loader]   Overlap at 20f000: merging perms to r=true w=false x=true
[12293566263] [INFO] [kernel::task::loader] Segment: vaddr=215300 exec=false
[12294615663] [INFO] [kernel::task::loader]   Overlap at 215000: merging perms to r=true w=true x=true
[12300172863] [INFO] [sprout::supervisor] SPROUT: App launched (PID=6)
[12300790029] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/cambium'
[12301638888] [INFO] [kernel::task::loader] Loading module: /boot/cambium
[12302118906] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[12303320733] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[12307049535] [INFO] [kernel::task::loader] Segment: vaddr=203e50 exec=false
[12307615353] [INFO] [kernel::task::loader]   Overlap at 203000: merging perms to r=true w=false x=true
[12308818500] [INFO] [kernel::task::loader] Segment: vaddr=204c88 exec=false
[12309362868] [INFO] [kernel::task::loader]   Overlap at 204000: merging perms to r=true w=true x=true
[12315040650] [INFO] [sprout::supervisor] SPROUT: App launched (PID=7)
[12315706623] [INFO] [sprout::pipelines] SPROUT: Setting up display pipeline...
[12329265630] [INFO] [clock] Clock thing created: 327
[12330110496] [INFO] [clock] Waiting for UI Root (Compositor)...
[12332763498] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0044678
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[12333819003] [INFO] [task.user_enter] Entering user mode tid=5 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132765 RSP_BEFORE=18446744072368012096 RFLAGS_BEFORE=130 CR3_BEFORE=50851840 fs_base=0 gs_base=18446744071563780584
[12338027196] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0045680
USER_TRAMPOLINE: PC=0x2083b0 SP=0x800000 ARG0=0x0
[12339058314] [INFO] [task.user_enter] Entering user mode tid=6 target_pc=2130864 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132765 RSP_BEFORE=18446744072368034864 RFLAGS_BEFORE=134 CR3_BEFORE=50958336 fs_base=0 gs_base=18446744071563780584
[12340953702] [ERROR] [INGESTD] Starting...
[12344244693] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0045718
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[12345221988] [INFO] [task.user_enter] Entering user mode tid=7 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132765 RSP_BEFORE=18446744072368051248 RFLAGS_BEFORE=134 CR3_BEFORE=51134464 fs_base=0 gs_base=18446744071563780584
[12347647851] [INFO] [cambium] cambium starting (v3: catch-up then stream)...
[12352674807] [INFO] [clock] UI Root not found yet (attempt 1), still waiting...
[12355796541] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7fef88
[12357264678] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[12358219434] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=0 start_seq=0
[12359458650] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=51 subj_lo=0
[12372871401] [ERROR] [INGESTD] Watch active. Loop start.
[12379871493] [INFO] [sprout::pipelines] SPROUT: Using boot framebuffer
[12380986398] [INFO] [sprout::pipelines] SPROUT: Display backend: BootFB (1280x720 stride=5120)
[12591400206] [INFO] [kernel::task::loader] Loading module: /boot/display_bootfb
[12592168743] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[12593308629] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[12596837220] [INFO] [kernel::task::loader] Segment: vaddr=202318 exec=false
[12597486990] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=false x=true
[12598459137] [INFO] [kernel::task::loader] Segment: vaddr=202f00 exec=false
[12599031324] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=true x=true
[12605653500] [INFO] [sprout::pipelines] SPROUT: Spawned display driver '/display_bootfb' (PID=8)
[12611831958] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb00141a8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x30002
[12612971778] [INFO] [task.user_enter] Entering user mode tid=8 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132765 RSP_BEFORE=18446744072368100640 RFLAGS_BEFORE=130 CR3_BEFORE=54927360 fs_base=0 gs_base=18446744071563780584
[12616244454] [INFO] [display_bootfb] display_bootfb: starting (drv_req_r=2, drv_resp_w=3)
[12633381882] [INFO] [kernel::syscall::handlers::device] DEVICE: task 8 claimed device 199 (handle 0)
[12642320559] [INFO] [kernel::syscall::handlers::device] DEVICE: Mapped BAR0 phys=0x80000000 size=0x384000 -> virt=0x10000000
[12650623953] [INFO] [sprout::supervisor] SPROUT: Found match for RTC: driver '/boot/rtc_cmos'
[12651699423] [INFO] [kernel::task::loader] Loading module: /boot/rtc_cmos
[12652269795] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[12653299725] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[12656387964] [INFO] [kernel::task::loader] Segment: vaddr=202c20 exec=false
[12657001500] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=false x=true
[12658321797] [INFO] [kernel::task::loader] Segment: vaddr=203b10 exec=false
[12658942065] [INFO] [kernel::task::loader]   Overlap at 203000: merging perms to r=true w=true x=true
[12665498769] [INFO] [sprout::supervisor] SPROUT: Driver launched (PID=9)
[12666596415] [INFO] [sprout::pipelines] SPROUT: Setting up input pipeline (keyboard + mouse)...
[12667815402] [INFO] [sprout::pipelines] SPROUT: Created kbd_raw port (w=5, r=6)
[12668870214] [INFO] [sprout::pipelines] SPROUT: Created mouse_raw port (w=7, r=8)
[12670178334] [INFO] [sprout::pipelines] SPROUT: Created evt port (w=9, r=10)
[12671102928] [INFO] [kernel::task::loader] Loading module: /boot/ps2_kbd
[12671597334] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[12672761673] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[12675048243] [INFO] [kernel::task::loader] Segment: vaddr=2015b0 exec=false
[12675640428] [INFO] [kernel::task::loader]   Overlap at 201000: merging perms to r=true w=false x=true
[12676536510] [INFO] [kernel::task::loader] Segment: vaddr=201f40 exec=false
[12677092923] [INFO] [kernel::task::loader]   Overlap at 201000: merging perms to r=true w=true x=true
[12682781595] [INFO] [sprout::pipelines] SPROUT: Spawned ps2_kbd (PID=10)
[12683971938] [INFO] [kernel::task::loader] Loading module: /boot/ps2_mouse
[12684529275] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[12685754598] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[12688729680] [INFO] [kernel::task::loader] Segment: vaddr=2026c0 exec=false
[12689309589] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=false x=true
[12690521448] [INFO] [kernel::task::loader] Segment: vaddr=2032d0 exec=false
[12691083636] [INFO] [kernel::task::loader]   Overlap at 203000: merging perms to r=true w=true x=true
[12696575859] [INFO] [sprout::pipelines] SPROUT: Spawned ps2_mouse (PID=11)
[12698424156] [INFO] [sprout::pipelines] SPROUT: Created evt_echo port (w=11, r=12)
[12699819891] [INFO] [kernel::task::loader] Loading module: /boot/bristle
[12700330500] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[12701243313] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[12704476752] [INFO] [kernel::task::loader] Segment: vaddr=2020e0 exec=false
[12705034551] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=false x=true
[12705938190] [INFO] [kernel::task::loader] Segment: vaddr=202ef8 exec=false
[12706479720] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=true x=true
[12712566339] [INFO] [sprout::pipelines] SPROUT: Spawned bristle (PID=12)
[12726286452] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0014318
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xdb
[12727338723] [INFO] [task.user_enter] Entering user mode tid=9 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132765 RSP_BEFORE=18446744072368124368 RFLAGS_BEFORE=130 CR3_BEFORE=55033856 fs_base=0 gs_base=18446744071563780584
[12731478474] [INFO] [rtc_cmos] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ffed0 rip=0x200037 rflags=0x202
[12732324594] [INFO] [rtc_cmos] Starting... arg=db
[12733718250] [INFO] [rtc_cmos] Serving device ID: ThingId([219, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[12736705740] [INFO] [rtc_cmos] RTC: 2026-01-21 01:50:23 = 1768960223 unix_secs
[12737790978] [INFO] [kernel::time] System clock anchored: unix_secs=1768960223, mono_ns=6368674999, offset=1768960216631325001ns
[12738930930] [INFO] [rtc_cmos] System clock anchored
[12753751626] [INFO] [rtc_cmos] Publishing time. Entering maintenance loop.
[12755937084] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0045dc0
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x5
[12756881214] [INFO] [task.user_enter] Entering user mode tid=10 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132765 RSP_BEFORE=18446744072368158944 RFLAGS_BEFORE=130 CR3_BEFORE=55136256 fs_base=0 gs_base=18446744071563780584
[12759995259] [INFO] [ps2_kbd] ps2_kbd: online (handle=5)
[12761763927] [INFO] [kernel::syscall::handlers::device] DEVICE: task subscribed to vector 0x21
[12762812436] [INFO] [ps2_kbd] ps2_kbd: subscribed to IRQ1 (vector 0x21)
[12764674626] [INFO] [ps2_kbd] ps2_kbd: entering interrupt-driven loop
[12769431609] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb004ba00
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x7
[12770569779] [INFO] [task.user_enter] Entering user mode tid=11 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132765 RSP_BEFORE=18446744072368176432 RFLAGS_BEFORE=134 CR3_BEFORE=55230464 fs_base=0 gs_base=18446744071563780584
[12773608749] [INFO] [ps2_mouse] ps2_mouse: online (handle=7)
[12774399891] [INFO] [ps2_mouse] ps2_mouse: enabling aux port
[12778321974] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb009c570
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600080009000b
[12779415462] [INFO] [task.user_enter] Entering user mode tid=12 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132765 RSP_BEFORE=18446744072368212528 RFLAGS_BEFORE=134 CR3_BEFORE=55332864 fs_base=0 gs_base=18446744071563780584
[12782556105] [INFO] [bristle] bristle: online (kbd=6, mouse=8, evt=9, evt_echo=11)
[12788740701] [INFO] [bristle] bristle: registered in graph as svc.Input (id=457)
[12796766796] [INFO] [sprout::pipelines] SPROUT: Bloom handles via BS=431 backend=BootFB
[12798416334] [INFO] [kernel::task::loader] Loading module: /boot/bloom
[12799037196] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[12800018715] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[12861466827] [INFO] [kernel::task::loader] Segment: vaddr=26b530 exec=false
[12862152732] [INFO] [kernel::task::loader]   Overlap at 26b000: merging perms to r=true w=false x=true
[12869967858] [INFO] [kernel::task::loader] Segment: vaddr=277420 exec=false
[12870604758] [INFO] [kernel::task::loader]   Overlap at 277000: merging perms to r=true w=true x=true
[12876511791] [INFO] [sprout::pipelines] SPROUT: Spawned bloom (PID=13)
[12877749687] [INFO] [kernel::task::loader] Loading module: /boot/echo
[12878285178] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[12879237624] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[12882086151] [INFO] [kernel::task::loader] Segment: vaddr=202110 exec=false
[12882688896] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=false x=true
[12884019522] [INFO] [kernel::task::loader] Segment: vaddr=202e58 exec=false
[12884597088] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=true x=true
[12890917908] [INFO] [sprout::pipelines] SPROUT: Spawned echo (PID=14)
[12891811977] [INFO] [sprout::pipelines] SPROUT: Input pipeline ready (keyboard + mouse)
[12892520586] [INFO] [sprout::supervisor] SPROUT: Entering supervisor loop.
[12902747979] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0014c50
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x1af
[12903798732] [INFO] [task.user_enter] Entering user mode tid=13 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132765 RSP_BEFORE=18446744072368250256 RFLAGS_BEFORE=134 CR3_BEFORE=55439360 fs_base=0 gs_base=18446744071563780584
[12906479124] [INFO] [bloom::logging] bloom: logging initialized
[12928917474] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb004b958
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xc
[12930022017] [INFO] [task.user_enter] Entering user mode tid=14 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132765 RSP_BEFORE=18446744072368267808 RFLAGS_BEFORE=130 CR3_BEFORE=56016896 fs_base=0 gs_base=18446744071563780584
[12934094811] [INFO] [echo] echo: online (handle=12)
[12934826751] [INFO] [echo] echo: ready for Bristle events (keyboard + mouse)
[12940810014] [INFO] [bloom] bloom: [bloom] Bootstrapped via Bytespace ID=431
[12943692861] [INFO] [bloom] bloom: [bloom] starting (arg_req=1 arg_resp=4 bristle=10 font_svc=0)
[12952846335] [INFO] [bloom] bloom: [bloom] spawned wallpaper_loader thread (tid=15)
[12959243154] [INFO] [bloom] bloom: [bloom] spawned cursor_loader thread (tid=16)
[12965993073] [INFO] [bloom] bloom: [bloom] spawned font_loader thread (tid=17)
[12966681123] [INFO] [bloom] bloom: [bloom] discovering compositor target...
T:DB40 [12971575320] [INFO] [bloom] bloom: [wallpaper_loader] thread started
T:D770 [12973256241] [INFO] [bloom] bloom: [cursor_loader] thread started
[12976959534] [INFO] [ps2_mouse] ps2_mouse: controller cfg already correct (0x47)
[12978189411] [INFO] [ps2_mouse] ps2_mouse: sending enable command (0xF4)
[12979252044] [INFO] [ps2_mouse] ps2_mouse: enable ACK received (0xFA)
T:CD00 [12982464792] [INFO] [bloom] bloom: [font_loader] thread started
[12983207094] [INFO] [bloom] bloom: [font_loader] scanning boot modules for fonts...
[12988971699] [INFO] [bloom] bloom: [font_loader] found 42 boot modules
[13078224159] [INFO] [bloom] bloom: [font_loader] found font module: '/assets/fonts/DSEG7Classic-Regular.ttf'
[13084967511] [INFO] [bloom] bloom: [font_loader] enqueuing immediate font load: bs=179 size=23272 name='/assets/fonts/DSEG7Classic-Regular.ttf'
[13093243482] [INFO] [bloom::asset] [asset_bank] worker spawned tid=18 (priority=2)
T:CB60 [13097711352] [INFO] [bloom::asset] [asset_bank] worker started (priority bump)
[13098790650] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 179 (23272 bytes)
[13101444048] [INFO] [bloom] bloom: [font_loader] found font module: '/assets/fonts/Hack-Regular.ttf'
[13105423089] [INFO] [bloom::asset] [asset_bank] mapped at 0x10386000
[13106188128] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/DSEG7Classic-Regular.ttf'...
[13128327663] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[13130744748] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'DSEG7Classic-Regular.ttf' in slot 0
[13137455595] [INFO] [bloom] bloom: [font_loader] enqueuing immediate font load: bs=182 size=309408 name='/assets/fonts/Hack-Regular.ttf'
[13141821957] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 182 (309408 bytes)
[13144207824] [INFO] [bloom] bloom: [font_loader] found font module: '/assets/fonts/NotoSans-Regular.ttf'
[13148650647] [INFO] [bloom::asset] [asset_bank] mapped at 0x1038c000
[13149393477] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/Hack-Regular.ttf'...
[13722091581] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[13723314462] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'Hack-Regular.ttf' in slot 1
[13726456524] [INFO] [bloom] bloom: [font_loader] enqueuing immediate font load: bs=185 size=569208 name='/assets/fonts/NotoSans-Regular.ttf'
[13731710817] [INFO] [bloom] bloom: [wallpaper_loader] searching for wallpaper...
[13732613268] [INFO] [bloom] bloom: [wallpaper_loader] trying: /assets/wallpapers/clouds.bmp
[13736397213] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 185 (569208 bytes)
[13738477830] [INFO] [bloom] bloom: [font_loader] found font module: '/assets/fonts/NotoSansSymbol-Regular.ttf'
[13744423968] [INFO] [bloom::asset] [asset_bank] mapped at 0x103d8000
[13750710204] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/NotoSans-Regular.ttf'...
[14077709712] [INFO] [bloom] bloom: [font_loader] enqueuing immediate font load: bs=188 size=258156 name='/assets/fonts/NotoSansSymbol-Regular.ttf'
[14372635002] [INFO] [bloom] bloom: [cursor_loader] searching for cursor...
[14373590220] [INFO] [bloom] bloom: [cursor_loader] trying: /assets/cursors/plain/Normal.cur
[15024437769] [INFO] [bloom] bloom: [font_loader] found font module: '/assets/fonts/NotoSansSymbol2-Regular.ttf'
[15677294160] [INFO] [bloom] bloom: [font_loader] enqueuing immediate font load: bs=191 size=656852 name='/assets/fonts/NotoSansSymbol2-Regular.ttf'
[16006403391] [INFO] [bloom::compositor] bloom: compositor bytespace 376 (1280x720 stride=5120 format=1)
[16971601350] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[16972817136] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'NotoSans-Regular.ttf' in slot 2
[16998780084] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 188 (258156 bytes)
[17001616533] [INFO] [bloom] bloom: [font_loader] found font module: '/assets/fonts/NotoSerif-Regular.ttf'
[17003915940] [INFO] [bloom::compositor] bloom: display backend: BootFB
[17008832016] [INFO] [bloom::asset] [asset_bank] mapped at 0x10463000
[17009573592] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/NotoSansSymbol-Regular.ttf'...
[17315991627] [INFO] [bloom::compositor] bloom: mapped size=3686400 (source=bytespace_info)
[17320832331] [INFO] [ps2_mouse] ps2_mouse: init done
[17321476260] [INFO] [kernel::syscall::handlers::device] DEVICE: task subscribed to vector 0x2c
[17322308454] [INFO] [ps2_mouse] ps2_mouse: subscribed to IRQ12 (vector 0x2c)
[17323034355] [INFO] [ps2_mouse] ps2_mouse: entering interrupt-driven loop
[17640851679] [INFO] [bloom] bloom: [font_loader] enqueuing immediate font load: bs=194 size=616196 name='/assets/fonts/NotoSerif-Regular.ttf'
[17649701751] [INFO] [bloom] bloom: [bloom] compositor target: 1280x720 @ 0x104a3000 backend=BootFB
[17650735740] [INFO] [bloom] bloom: [bloom] presenter: driver (req=1 resp=4)
[17652178434] [INFO] [bloom::frame_loop] bloom: running (fps_target=60)
[17968713906] [INFO] [bloom] bloom: [font_loader] immediate scan complete, entering watch loop
[17971563390] [INFO] [stem::ui] UiBuilder: created root 545
[17972264607] [INFO] [bloom] bloom: [bloom] created UI root node: 545
[17980889124] [INFO] [display_bootfb] display_bootfb: bound bytespace 376
[18296890326] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7fd5b0
[18297874518] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[18298532307] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[18299696415] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: NO FILTER (filter_ptr=0x0 filter_len=0)
[18555084768] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[18556719588] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'NotoSansSymbol-Regular.ttf' in slot 3
[18557547789] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 191 (656852 bytes)
[18561445650] [INFO] [bloom] bloom: [bloom] ui watch opened UNFILTERED (userspace filter pred=UI_TEXT id=519)
[18568781979] [INFO] [bloom::asset] [asset_bank] mapped at 0x10bab000
[18569419374] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/NotoSansSymbol2-Regular.ttf'...
[18954591282] [INFO] [clock] Found UI Root: 545 (attempt 4)
[22222781031] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x100c1b40
[22224091923] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[22225161057] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[22226042388] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: NO FILTER (filter_ptr=0x0 filter_len=0)
[22568725674] [INFO] [bloom] bloom: [font_loader] watch opened (id=572)
[23958229977] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[23959241922] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'NotoSansSymbol2-Regular.ttf' in slot 4
[23960164998] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 194 (616196 bytes)
[23967204624] [INFO] [bloom] bloom: [cursor_loader] SUCCESS: found candidate '/assets/cursors/plain/Normal.cur', enqueuing load
[23968461099] [INFO] [bloom] bloom: [cursor_loader] thread done, sleeping forever
[23975320314] [INFO] [bloom::asset] [asset_bank] mapped at 0x10c4c000
[23976021630] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/NotoSerif-Regular.ttf'...
[24845099532] [INFO] [bloom] bloom: [wallpaper_loader] SUCCESS: found candidate '/assets/wallpapers/clouds.bmp', enqueuing load
[26811664803] [INFO] [bloom] bloom: [wallpaper_loader] thread done, sleeping forever
[29753455347] [INFO] [clock] Binding created: 590 (source=327 target=575)
[29754349251] [INFO] [clock] CLOCK: Entering main loop, publishing to thing_id=327
[29756759802] [INFO] [clock] unix=1768960231 utc=2026-01-21 01:50:31 mono_ns=14877666969
[31064474364] [INFO] [clock] CLOCK PUBLISH: thing=327 now_text='01:50:31' tick=14877666969
[31393835352] [INFO] [cambium] Found 1 bindings
[32374665477] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7fedd0
[32375528724] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[32376354087] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[32377217235] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x4 kind=0 pred=0 subj_lo=327
[32701692969] [INFO] [cambium] Opened watch 607 for source 327 (binding 590, start_seq=0)
[32725388487] [INFO] [cambium] CATCH-UP: Draining 1 watches for historical events...
[33069355671] [INFO] [cambium] cambium: drain complete payloads=0 overflows=0 last_seq=none
[33070342206] [INFO] [cambium] Entering event loop with 1 bindings (v3)
[34372668726] [INFO] [clock] unix=1768960233 utc=2026-01-21 01:50:33 mono_ns=17186115903
[35680810935] [INFO] [clock] CLOCK PUBLISH: thing=327 now_text='01:50:33' tick=17186115903

```
</details>
