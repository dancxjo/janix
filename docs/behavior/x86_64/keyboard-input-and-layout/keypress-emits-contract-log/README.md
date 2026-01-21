# ❌ Scenario: Keypress emits contract log

> Last run: 2026-01-20 19:32:43

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the clock window is ticking | ✅ | 13039ms | <a href="./01/after.png"><img src="./01/after.png" width="150" /></a> - [💾](./01/registers.txt) |
| 2 | When I press a key | ❌ | 1014ms | - [📜](./02/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[11086621194] [CONTRACT] [kernel] thing-os kernel starting...
[11097072294] [INFO] [kernel::memory] Memory map has 64 entries
[11099292996] [INFO] [kernel::memory]   [0] 0x0 - 0xa0000 (Usable)
[11099908380] [INFO] [kernel::memory]   [1] 0x100000 - 0x800000 (Usable)
[11100243594] [INFO] [kernel::memory]   [2] 0x800000 - 0x808000 (Other)
[11100604746] [INFO] [kernel::memory]   [3] 0x808000 - 0x80b000 (Usable)
[11100932139] [INFO] [kernel::memory]   [4] 0x80b000 - 0x80c000 (Other)
[11101255836] [INFO] [kernel::memory]   [5] 0x80c000 - 0x811000 (Usable)
[11101649889] [INFO] [kernel::memory]   [6] 0x811000 - 0x900000 (Other)
[11101978371] [INFO] [kernel::memory]   [7] 0x900000 - 0x1780000 (Reserved)
[11102407668] [INFO] [kernel::memory]   [8] 0x1780000 - 0x78656000 (Usable)
[11102846469] [INFO] [kernel::memory]   [9] 0x78656000 - 0x786b8000 (Reserved)
[11103200955] [INFO] [kernel::memory]   [10] 0x786b8000 - 0x7883a000 (Other)
[11103545673] [INFO] [kernel::memory]   [11] 0x7883a000 - 0x7883b000 (Reserved)
[11103901380] [INFO] [kernel::memory]   [12] 0x7883b000 - 0x788d2000 (Other)
[11104245570] [INFO] [kernel::memory]   [13] 0x788d2000 - 0x788d3000 (Reserved)
[11104601211] [INFO] [kernel::memory]   [14] 0x788d3000 - 0x78974000 (Other)
[11104943817] [INFO] [kernel::memory]   [15] 0x78974000 - 0x78975000 (Reserved)
[11105298303] [INFO] [kernel::memory]   [16] 0x78975000 - 0x789b5000 (Other)
[11105662392] [INFO] [kernel::memory]   [17] 0x789b5000 - 0x789b6000 (Reserved)
[11106019683] [INFO] [kernel::memory]   [18] 0x789b6000 - 0x78a41000 (Other)
[11109528144] [INFO] [kernel::memory]   [19] 0x78a41000 - 0x78a42000 (Reserved)
[11109969651] [INFO] [kernel::memory]   [20] 0x78a42000 - 0x78a8e000 (Other)
[11110627011] [INFO] [kernel::memory]   [21] 0x78a8e000 - 0x78a8f000 (Reserved)
[11110993212] [INFO] [kernel::memory]   [22] 0x78a8f000 - 0x78a95000 (Other)
[11111339943] [INFO] [kernel::memory]   [23] 0x78a95000 - 0x78a96000 (Reserved)
[11111698158] [INFO] [kernel::memory]   [24] 0x78a96000 - 0x78f17000 (Other)
[11112065481] [INFO] [kernel::memory]   [25] 0x78f17000 - 0x78f18000 (Reserved)
[11112490092] [INFO] [kernel::memory]   [26] 0x78f18000 - 0x79399000 (Other)
[11112838044] [INFO] [kernel::memory]   [27] 0x79399000 - 0x7939a000 (Reserved)
[11113304037] [INFO] [kernel::memory]   [28] 0x7939a000 - 0x7969b000 (Other)
[11113779600] [INFO] [kernel::memory]   [29] 0x7969b000 - 0x7969c000 (Reserved)
[11114140884] [INFO] [kernel::memory]   [30] 0x7969c000 - 0x796a5000 (Other)
[11114486526] [INFO] [kernel::memory]   [31] 0x796a5000 - 0x796a6000 (Reserved)
[11114914338] [INFO] [kernel::memory]   [32] 0x796a6000 - 0x796aa000 (Other)
[11115263346] [INFO] [kernel::memory]   [33] 0x796aa000 - 0x796ab000 (Reserved)
[11115640569] [INFO] [kernel::memory]   [34] 0x796ab000 - 0x796ad000 (Other)
[11115986937] [INFO] [kernel::memory]   [35] 0x796ad000 - 0x796ae000 (Reserved)
[11116344459] [INFO] [kernel::memory]   [36] 0x796ae000 - 0x796b0000 (Other)
[11116689474] [INFO] [kernel::memory]   [37] 0x796b0000 - 0x796b1000 (Reserved)
[11117048844] [INFO] [kernel::memory]   [38] 0x796b1000 - 0x796b3000 (Other)
[11117495169] [INFO] [kernel::memory]   [39] 0x796b3000 - 0x796b4000 (Reserved)
[11118014424] [INFO] [kernel::memory]   [40] 0x796b4000 - 0x796b6000 (Other)
[11118546978] [INFO] [kernel::memory]   [41] 0x796b6000 - 0x796b7000 (Reserved)
[11118952845] [INFO] [kernel::memory]   [42] 0x796b7000 - 0x796b9000 (Other)
[11119380888] [INFO] [kernel::memory]   [43] 0x796b9000 - 0x796ba000 (Reserved)
[11119833054] [INFO] [kernel::memory]   [44] 0x796ba000 - 0x796c4000 (Other)
[11120239251] [INFO] [kernel::memory]   [45] 0x796c4000 - 0x796c5000 (Reserved)
[11120643732] [INFO] [kernel::memory]   [46] 0x796c5000 - 0x796c9000 (Other)
[11121061050] [INFO] [kernel::memory]   [47] 0x796c9000 - 0x796ca000 (Reserved)
[11121487773] [INFO] [kernel::memory]   [48] 0x796ca000 - 0x796cc000 (Other)
[11121834471] [INFO] [kernel::memory]   [49] 0x796cc000 - 0x796cd000 (Reserved)
[11122228623] [INFO] [kernel::memory]   [50] 0x796cd000 - 0x796cf000 (Other)
[11122643334] [INFO] [kernel::memory]   [51] 0x796cf000 - 0x796d0000 (Reserved)
[11123069727] [INFO] [kernel::memory]   [52] 0x796d0000 - 0x796d4000 (Other)
[11123420385] [INFO] [kernel::memory]   [53] 0x796d4000 - 0x796d5000 (Reserved)
[11123776719] [INFO] [kernel::memory]   [54] 0x796d5000 - 0x79750000 (Other)
[11124122724] [INFO] [kernel::memory]   [55] 0x79750000 - 0x79907000 (Other)
[11124517800] [INFO] [kernel::memory]   [56] 0x79907000 - 0x7a16c000 (Reserved)
[11124876510] [INFO] [kernel::memory]   [57] 0x7a16c000 - 0x7bb6c000 (Usable)
[11125229511] [INFO] [kernel::memory]   [58] 0x7bb6c000 - 0x7bb8c000 (Reserved)
[11125611090] [INFO] [kernel::memory]   [59] 0x7bb8c000 - 0x7bb90000 (Other)
[11125957194] [INFO] [kernel::memory]   [60] 0x7bb90000 - 0x7bb91000 (Reserved)
[11126443548] [INFO] [kernel::memory]   [61] 0x7bb91000 - 0x7bb93000 (Other)
[11126791467] [INFO] [kernel::memory]   [62] 0x7bb93000 - 0x7bb94000 (Reserved)
[11127153246] [INFO] [kernel::memory]   [63] 0x7bb94000 - 0x7bb96000 (Other)
[11127783414] [INFO] [kernel::memory] HHDM Offset: 0xffff800000000000
[11372039217] [CONTRACT] [kernel::memory] Frame allocator initialized with 495726 free frames
[11379342348] [INFO] [bran::arch] IOAPIC: hhdm=0xffff800000000000
[11383800648] [INFO] [bran::arch] IOAPIC: Disabling legacy PIC...
[11384859255] [INFO] [bran::arch] IOAPIC: PIC disabled OK
[11385519948] [INFO] [bran::arch] IOAPIC: RSDP virt=0x7f77e014
[11393882676] [INFO] [bran::arch] IOAPIC: MADT parsed OK
[11394371736] [INFO] [bran::arch] IOAPIC: Found at phys 0xfec00000, GSI base 0
[11397154065] [INFO] [bran::arch] IOAPIC: Registers initialized
[11398514160] [INFO] [bran::arch] IOAPIC: version 0x20, 24 redir entries
[11400124395] [INFO] [bran::arch] IOAPIC: All pins masked
[11401400802] [INFO] [bran::arch] IOAPIC: IRQ1 -> GSI 1 -> 0x21
[11402000280] [INFO] [bran::arch] IOAPIC: IRQ12 -> GSI 12 -> 0x2C
[11402503068] [INFO] [bran::arch] IOAPIC: Init complete
[11403097728] [CONTRACT] [kernel] Initializing global allocator...
[11747806851] [INFO] [kernel::memory::global_alloc] Global allocator initialized (LinkedHeap, 32MB)
[11748704187] [CONTRACT] [kernel] Initializing SIMD...
[11750208921] [CONTRACT] [kernel] Initializing tasking...
[11754909408] [INFO] [kernel::task::scheduler]   Acquiring scheduler lock...
[11756451762] [INFO] [kernel::task::scheduler]   Lock acquired, checking if initialized...
[11757019527] [INFO] [kernel::task::scheduler]   Allocating scheduler...
[11762862144] [INFO] [kernel::task::scheduler]   Leaking scheduler...
[11763275535] [INFO] [kernel::task::scheduler]   Initializing boot task...
[11763985167] [INFO] [kernel::task::scheduler]   Creating boot task...
[11768291997] [INFO] [kernel::task::scheduler]   Creating idle task...
[11773047957] [INFO] [kernel::task::scheduler]   Boot task initialized
[11773447719] [INFO] [kernel::task::scheduler]   Storing scheduler pointer...
[11774320503] [CONTRACT] [kernel::task::scheduler] Scheduler initialized
[11780717718] [INFO] [kernel::root] Spawning Root service...
[11788302207] [INFO] [kernel::root::boot_register] ROOT: boot registration begin (Census Phase 1 v0.2)
[11797010445] [INFO] [kernel::root::service] ROOT: started once
[12744963528] [INFO] [kernel::root::boot_register] ROOT: Census Phase 2: PCI
[12745836741] [INFO] [kernel::root::pci] PCI: Starting enumeration...
[12792525570] [INFO] [kernel::root::pci] PCI: 00:00.0 8086:29c0 Intel Corporation 82G33/G31/P35/P31 Express DRAM Controller class=06:00 prog_if=00 rev=00
[12811707282] [INFO] [kernel::root::pci] PCI: 00:01.0 1234:1111 (unknown vendor) (unknown device) class=03:00 prog_if=00 rev=02
[12841364877] [INFO] [kernel::root::pci] PCI: 00:02.0 8086:10d3 Intel Corporation 82574L Gigabit Network Connection class=02:00 prog_if=00 rev=00
[12878029263] [INFO] [kernel::root::pci] PCI: 00:1f.0 8086:2918 Intel Corporation 82801IB (ICH9) LPC Interface Controller class=06:01 prog_if=00 rev=02
[12879914124] [INFO] [kernel::root::pci] PCI: Found LPC/ISA bridge at 00:1f.0
[12920049087] [INFO] [kernel::root::pci] LPC: Created Legacy IO bus with CMOS and PS/2 controller
[12946864722] [INFO] [kernel::root::pci] PCI: 00:1f.2 8086:2922 Intel Corporation 82801IR/IO/IH (ICH9R/DO/DH) 6 port SATA Controller [AHCI mode] class=01:06 prog_if=01 rev=02
[12953601408] [INFO] [kernel::root::pci] PCI: Found AHCI SATA controller at 00:1f.2
[12956861577] [INFO] [kernel::root::pci] PCI: Registered AHCI controller (graph_id=224, idx=3) BAR5=0x810c4000
[12981204060] [INFO] [kernel::root::pci] PCI: 00:1f.3 8086:2930 Intel Corporation 82801I (ICH9 Family) SMBus Controller class=0c:05 prog_if=00 rev=02
[12985068327] [INFO] [kernel::root::boot_register] ROOT: registered items. host=3 kernel=6
[12986914149] [CONTRACT] [kernel] KERNEL: root census complete: host=t3 kernel=t6 root=t7
[12990313875] [INFO] [kernel] Found init module: /boot/sprout (cmdline: ''), loading...
[12993018423] [INFO] [kernel::task::loader] Loading module: /boot/sprout
[12993803361] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[13001464905] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[13014552606] [INFO] [kernel::task::loader] Segment: vaddr=20d230 exec=false
[13015633059] [INFO] [kernel::task::loader]   Overlap at 20d000: merging perms to r=true w=false x=true
[13018097202] [INFO] [kernel::task::loader] Segment: vaddr=2100a8 exec=false
[13018994538] [INFO] [kernel::task::loader]   Overlap at 210000: merging perms to r=true w=true x=true
[13026950970] [INFO] [kernel] Spawning sprout with registry at 0x600000...
[13027649778] [CONTRACT] [kernel] Spawning init process...
[13029185334] [INFO] [kernel] System initialized. Setting up preemption timer (100Hz)...
[13063885428] [INFO] [bran::arch::x86_64::ioapic] LAPIC: calibrated timer (62214500 ticks/sec), init_cnt=622145 for 100Hz
[13065909417] [CONTRACT] [kernel] Entering scheduler loop.
[13076946960] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb00084c8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600000
[13081795848] [INFO] [task.user_enter] Entering user mode tid=3 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132621 RSP_BEFORE=18446744072367676144 RFLAGS_BEFORE=134 CR3_BEFORE=50319360 fs_base=0 gs_base=18446744071563780584
[13092461382] [INFO] [sprout] [sprout] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ff9a0 rip=0x20038f rflags=0x206
[13094519823] [INFO] [sprout] SPROUT: v0.4 starting (Supervisor Mode)...
[13095567309] [INFO] [sprout::devtree] SPROUT: devtree::init entry (v0.2)
[13096511274] [INFO] [sprout::devtree] SPROUT: Step 1: Find Host
[13117691235] [INFO] [sprout::devtree] SPROUT: Step 2: HHDM
[13121599326] [INFO] [sprout::devtree] SPROUT: Step 3: Platform Bus
[13125329316] [INFO] [sprout::devtree] SPROUT: Step 4: Firmware
[13126145802] [INFO] [sprout::devtree] SPROUT: Finding ACPI...
[13129942122] [INFO] [sprout::devtree] SPROUT: Found 1 ACPI nodes
[13132335315] [INFO] [sprout::devtree] SPROUT: ACPI RSDP = 0x7f77e014
[13133107020] [INFO] [sprout::devtree] SPROUT: Finding DTB...
[13136207799] [INFO] [sprout::devtree] SPROUT: Found 0 DTB nodes
[13136953665] [INFO] [sprout::devtree] SPROUT: Init OK, returning context
[13137656037] [INFO] [sprout::devtree] SPROUT: build() called
[13138334748] [INFO] [sprout::devtree::x86_64] SPROUT: x86_64 platform enrichment... (v0.2)
[13142878023] [INFO] [sprout::devtree::x86_64] SPROUT: x86_64 enumerate done
[13143632040] [INFO] [sprout] SPROUT: About to create Supervisor...
[13144278939] [INFO] [sprout] SPROUT: Supervisor created, calling run_forever...
[13145060478] [INFO] [sprout::supervisor] SPROUT: Supervisor starting...
[13145696850] [INFO] [sprout::supervisor] SPROUT: Discovering modules...
[13150534122] [INFO] [sprout::supervisor] SPROUT: Found 42 modules
[13188716970] [INFO] [sprout::supervisor] SPROUT: Module[0] = '/boot/sprout'
[13194208203] [INFO] [sprout::supervisor] SPROUT: Module[1] = '/boot/bristle'
[13198070919] [INFO] [sprout::supervisor] SPROUT: Module[2] = '/boot/rtc_cmos'
[13201489224] [INFO] [sprout::supervisor] SPROUT: Module[3] = '/boot/clock'
[13204504269] [INFO] [sprout::supervisor] SPROUT: Discovered app: /boot/clock
[13208302998] [INFO] [sprout::supervisor] SPROUT: Module[4] = '/boot/font_explorer'
[13211835912] [INFO] [sprout::supervisor] SPROUT: Module[5] = '/boot/ps2_kbd'
[13215135714] [INFO] [sprout::supervisor] SPROUT: Module[6] = '/boot/echo'
[13218330543] [INFO] [sprout::supervisor] SPROUT: Module[7] = '/boot/bloom'
[13221497850] [INFO] [sprout::supervisor] SPROUT: Module[8] = '/boot/ps2_mouse'
[13224560745] [INFO] [sprout::supervisor] SPROUT: Module[9] = '/boot/root_batch_bench'
[13228589121] [INFO] [sprout::supervisor] SPROUT: Module[10] = '/boot/root_watch_tester'
[13232859222] [INFO] [sprout::supervisor] SPROUT: Module[11] = '/boot/display_bootfb'
[13236605316] [INFO] [sprout::supervisor] SPROUT: Module[12] = '/boot/ingestd'
[13239652767] [INFO] [sprout::supervisor] SPROUT: Module[13] = '/boot/cambium'
[13242656361] [INFO] [sprout::supervisor] SPROUT: Module[14] = '/boot/scheduler_fairness'
[13245644643] [INFO] [sprout::supervisor] SPROUT: Module[15] = '/boot/hogger'
[13248607218] [INFO] [sprout::supervisor] SPROUT: Module[16] = '/boot/tick_printer'
[13252194252] [INFO] [sprout::supervisor] SPROUT: Module[17] = '/assets/cursors/plain/Alternate.cur'
[13256146893] [INFO] [sprout::supervisor] SPROUT: Module[18] = '/assets/cursors/plain/Busy.cur'
[13260051585] [INFO] [sprout::supervisor] SPROUT: Module[19] = '/assets/cursors/plain/Diagonal1.ani'
[13263932715] [INFO] [sprout::supervisor] SPROUT: Module[20] = '/assets/cursors/plain/Diagonal2.ani'
[13266981552] [INFO] [sprout::supervisor] SPROUT: Module[21] = '/assets/cursors/plain/Handwriting.cur'
[13270243404] [INFO] [sprout::supervisor] SPROUT: Module[22] = '/assets/cursors/plain/Help.cur'
[13273772424] [INFO] [sprout::supervisor] SPROUT: Module[23] = '/assets/cursors/plain/Horizontal.ani'
[13276894158] [INFO] [sprout::supervisor] SPROUT: Module[24] = '/assets/cursors/plain/Link.ani'
[13279940751] [INFO] [sprout::supervisor] SPROUT: Module[25] = '/assets/cursors/plain/Move.cur'
[13283036151] [INFO] [sprout::supervisor] SPROUT: Module[26] = '/assets/cursors/plain/Normal.cur'
[13286888934] [INFO] [sprout::supervisor] SPROUT: Module[27] = '/assets/cursors/plain/Precision.cur'
[13290155769] [INFO] [sprout::supervisor] SPROUT: Module[28] = '/assets/cursors/plain/Text.cur'
[13293410196] [INFO] [sprout::supervisor] SPROUT: Module[29] = '/assets/cursors/plain/Unavailabe.cur'
[13296938787] [INFO] [sprout::supervisor] SPROUT: Module[30] = '/assets/cursors/plain/Vertical.ani'
[13300506351] [INFO] [sprout::supervisor] SPROUT: Module[31] = '/assets/cursors/plain/Working.ani'
[13303623696] [INFO] [sprout::supervisor] SPROUT: Module[32] = '/assets/wallpapers/clouds.bmp'
[13306911057] [INFO] [sprout::supervisor] SPROUT: Module[33] = '/assets/wallpapers/leather.bmp'
[13310555643] [INFO] [sprout::supervisor] SPROUT: Module[34] = '/assets/wallpapers/linen.bmp'
[13314487197] [INFO] [sprout::supervisor] SPROUT: Module[35] = '/assets/fonts/DSEG7Classic-Regular.ttf'
[13318031760] [INFO] [sprout::supervisor] SPROUT: Module[36] = '/assets/fonts/Hack-Regular.ttf'
[13321175769] [INFO] [sprout::supervisor] SPROUT: Module[37] = '/assets/fonts/NotoSans-Regular.ttf'
[13324358850] [INFO] [sprout::supervisor] SPROUT: Module[38] = '/assets/fonts/NotoSansSymbol-Regular.ttf'
[13327863912] [INFO] [sprout::supervisor] SPROUT: Module[39] = '/assets/fonts/NotoSansSymbol2-Regular.ttf'
[13331175264] [INFO] [sprout::supervisor] SPROUT: Module[40] = '/assets/fonts/NotoSerif-Regular.ttf'
[13334697816] [INFO] [sprout::supervisor] SPROUT: Module[41] = '/assets/pci/pci.ids'
[13337072958] [INFO] [sprout::registry] SPROUT: Scanning boot modules...
[13350170130] [INFO] [sprout::registry] SPROUT: Registering driver 'dev.rtc.Cmos' -> '/boot/rtc_cmos' (fallback)
[13429217439] [INFO] [sprout::registry] SPROUT: Registry scan complete. Found 1 drivers.
[13430238261] [INFO] [sprout::supervisor] SPROUT: spawn_apps start. tasks len=1
[13432281885] [INFO] [sprout::supervisor] SPROUT: Adding fallback app '/boot/font_explorer'
[13433343594] [INFO] [sprout::supervisor] SPROUT: Adding fallback app '/boot/ingestd'
[13434116652] [INFO] [sprout::supervisor] SPROUT: Adding fallback app '/boot/cambium'
[13434986664] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/clock'
[13437726522] [INFO] [kernel::task::loader] Loading module: /boot/clock
[13438277358] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[13439428233] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[13443245013] [INFO] [kernel::task::loader] Segment: vaddr=203e50 exec=false
[13443829212] [INFO] [kernel::task::loader]   Overlap at 203000: merging perms to r=true w=false x=true
[13445168187] [INFO] [kernel::task::loader] Segment: vaddr=204ee8 exec=false
[13445736018] [INFO] [kernel::task::loader]   Overlap at 204000: merging perms to r=true w=true x=true
[13453066374] [INFO] [sprout::supervisor] SPROUT: App launched (PID=4)
[13456255857] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/font_explorer'
[13457295093] [INFO] [kernel::task::loader] Loading module: /boot/font_explorer
[13457916450] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[13459475403] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[13463541432] [INFO] [kernel::task::loader] Segment: vaddr=2041a0 exec=false
[13464198099] [INFO] [kernel::task::loader]   Overlap at 204000: merging perms to r=true w=false x=true
[13465045275] [INFO] [kernel::task::loader] Segment: vaddr=204668 exec=false
[13465624062] [INFO] [kernel::task::loader]   Overlap at 204000: merging perms to r=true w=true x=true
[13471631217] [INFO] [sprout::supervisor] SPROUT: App launched (PID=5)
[13472832912] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/ingestd'
[13473877461] [INFO] [kernel::task::loader] Loading module: /boot/ingestd
[13474391535] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[13475388531] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[13486689249] [INFO] [kernel::task::loader] Segment: vaddr=20f750 exec=false
[13487599521] [INFO] [kernel::task::loader]   Overlap at 20f000: merging perms to r=true w=false x=true
[13493744847] [INFO] [kernel::task::loader] Segment: vaddr=215300 exec=false
[13494349836] [INFO] [kernel::task::loader]   Overlap at 215000: merging perms to r=true w=true x=true
[13500341514] [INFO] [sprout::supervisor] SPROUT: App launched (PID=6)
[13501079163] [INFO] [sprout::supervisor] SPROUT: Launching app '/boot/cambium'
[13502508228] [INFO] [kernel::task::loader] Loading module: /boot/cambium
[13503032829] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[13504057050] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[13508587191] [INFO] [kernel::task::loader] Segment: vaddr=204360 exec=false
[13509276528] [INFO] [kernel::task::loader]   Overlap at 204000: merging perms to r=true w=false x=true
[13510691304] [INFO] [kernel::task::loader] Segment: vaddr=205168 exec=false
[13511305599] [INFO] [kernel::task::loader]   Overlap at 205000: merging perms to r=true w=true x=true
[13517545965] [INFO] [sprout::supervisor] SPROUT: App launched (PID=7)
[13518318132] [INFO] [sprout::pipelines] SPROUT: Setting up display pipeline...
[13535379759] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb00451d8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[13536737379] [INFO] [task.user_enter] Entering user mode tid=4 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132621 RSP_BEFORE=18446744072367986928 RFLAGS_BEFORE=134 CR3_BEFORE=50745344 fs_base=0 gs_base=18446744071563780584
[13540803804] [INFO] [clock] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ffea0 rip=0x2000cf rflags=0x206
[13541825385] [INFO] [clock] starting clock publisher
[13551931899] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0004178
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[13553166000] [INFO] [task.user_enter] Entering user mode tid=5 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132621 RSP_BEFORE=18446744072368009968 RFLAGS_BEFORE=134 CR3_BEFORE=50851840 fs_base=0 gs_base=18446744071563780584
[13558152630] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0045ab8
USER_TRAMPOLINE: PC=0x2083b0 SP=0x800000 ARG0=0x0
[13559343666] [INFO] [task.user_enter] Entering user mode tid=6 target_pc=2130864 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132621 RSP_BEFORE=18446744072368026352 RFLAGS_BEFORE=134 CR3_BEFORE=50958336 fs_base=0 gs_base=18446744071563780584
[13561826157] [ERROR] [INGESTD] Starting...
[13565611092] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0045ae8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[13566796221] [INFO] [task.user_enter] Entering user mode tid=7 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132621 RSP_BEFORE=18446744072368053488 RFLAGS_BEFORE=134 CR3_BEFORE=51134464 fs_base=0 gs_base=18446744071563780584
[13569483807] [INFO] [cambium] cambium starting (v4: no-op suppression)...
[13575523731] [INFO] [clock] Clock thing created: 356
[13576258542] [INFO] [clock] Waiting for UI Root (Compositor)...
[13581204780] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7fef88
[13582453137] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[13583538639] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=0 start_seq=0
[13584851841] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=51 subj_lo=0
[13608049752] [INFO] [clock] UI Root not found yet (attempt 1), still waiting...
[13617095019] [ERROR] [INGESTD] Watch active. Loop start.
[13626386400] [INFO] [sprout::pipelines] SPROUT: Using boot framebuffer
[13627715607] [INFO] [sprout::pipelines] SPROUT: Display backend: BootFB (1280x720 stride=5120)
[13862275515] [INFO] [kernel::task::loader] Loading module: /boot/display_bootfb
[13863810477] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[13866205815] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[13869999825] [INFO] [kernel::task::loader] Segment: vaddr=202318 exec=false
[13870620951] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=false x=true
[13871599302] [INFO] [kernel::task::loader] Segment: vaddr=202f00 exec=false
[13872216765] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=true x=true
[13878989850] [INFO] [sprout::pipelines] SPROUT: Spawned display driver '/display_bootfb' (PID=8)
[13886452536] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0004178
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x30002
[13887737259] [INFO] [task.user_enter] Entering user mode tid=8 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132621 RSP_BEFORE=18446744072368106224 RFLAGS_BEFORE=134 CR3_BEFORE=54931456 fs_base=0 gs_base=18446744071563780584
[13892412006] [INFO] [display_bootfb] display_bootfb: starting (drv_req_r=2, drv_resp_w=3)
[13910827425] [INFO] [kernel::syscall::handlers::device] DEVICE: task 8 claimed device 199 (handle 0)
[13919989875] [INFO] [kernel::syscall::handlers::device] DEVICE: Mapped BAR0 phys=0x80000000 size=0x384000 -> virt=0x10000000
[13931002866] [INFO] [sprout::supervisor] SPROUT: Found match for RTC: driver '/boot/rtc_cmos'
[13932912246] [INFO] [kernel::task::loader] Loading module: /boot/rtc_cmos
[13933476744] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[13934595609] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[13938140700] [INFO] [kernel::task::loader] Segment: vaddr=202c20 exec=false
[13938903396] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=false x=true
[13940242635] [INFO] [kernel::task::loader] Segment: vaddr=203b10 exec=false
[13940813304] [INFO] [kernel::task::loader]   Overlap at 203000: merging perms to r=true w=true x=true
[13947584376] [INFO] [sprout::supervisor] SPROUT: Driver launched (PID=9)
[13948971036] [INFO] [sprout::pipelines] SPROUT: Setting up input pipeline (keyboard + mouse)...
[13951094025] [INFO] [sprout::pipelines] SPROUT: Created kbd_raw port (w=5, r=6)
[13952360598] [INFO] [sprout::pipelines] SPROUT: Created mouse_raw port (w=7, r=8)
[13954050396] [INFO] [sprout::pipelines] SPROUT: Created evt port (w=9, r=10)
[13955331126] [INFO] [kernel::task::loader] Loading module: /boot/ps2_kbd
[13956373134] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[13957632480] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[13960126917] [INFO] [kernel::task::loader] Segment: vaddr=2015b0 exec=false
[13960708344] [INFO] [kernel::task::loader]   Overlap at 201000: merging perms to r=true w=false x=true
[13961750121] [INFO] [kernel::task::loader] Segment: vaddr=201f40 exec=false
[13962323661] [INFO] [kernel::task::loader]   Overlap at 201000: merging perms to r=true w=true x=true
[13968611250] [INFO] [sprout::pipelines] SPROUT: Spawned ps2_kbd (PID=10)
[13969996326] [INFO] [kernel::task::loader] Loading module: /boot/ps2_mouse
[13970546667] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[13971575541] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[13974599793] [INFO] [kernel::task::loader] Segment: vaddr=2026c0 exec=false
[13975195146] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=false x=true
[13976631933] [INFO] [kernel::task::loader] Segment: vaddr=2032d0 exec=false
[13977218013] [INFO] [kernel::task::loader]   Overlap at 203000: merging perms to r=true w=true x=true
[13984278297] [INFO] [sprout::pipelines] SPROUT: Spawned ps2_mouse (PID=11)
[13986441150] [INFO] [sprout::pipelines] SPROUT: Created evt_echo port (w=11, r=12)
[13987519755] [INFO] [kernel::task::loader] Loading module: /boot/bristle
[13988063298] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[13989078741] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[13991896743] [INFO] [kernel::task::loader] Segment: vaddr=2020e0 exec=false
[13992456456] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=false x=true
[13993514766] [INFO] [kernel::task::loader] Segment: vaddr=202ef8 exec=false
[13994107446] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=true x=true
[14000459154] [INFO] [sprout::pipelines] SPROUT: Spawned bristle (PID=12)
[14016045648] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0044678
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xdb
[14017370235] [INFO] [task.user_enter] Entering user mode tid=9 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132621 RSP_BEFORE=18446744072368129888 RFLAGS_BEFORE=134 CR3_BEFORE=55037952 fs_base=0 gs_base=18446744071563780584
[14021826852] [INFO] [rtc_cmos] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ffed0 rip=0x200037 rflags=0x202
[14022755835] [INFO] [rtc_cmos] Starting... arg=db
[14024318253] [INFO] [rtc_cmos] Serving device ID: ThingId([219, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[14027790645] [INFO] [rtc_cmos] RTC: 2026-01-21 03:33:13 = 1768966393 unix_secs
[14029111536] [INFO] [kernel::time] System clock anchored: unix_secs=1768966393, mono_ns=7014306255, offset=1768966385985693745ns
[14030436420] [INFO] [rtc_cmos] System clock anchored
[14051875596] [INFO] [rtc_cmos] Publishing time. Entering maintenance loop.
[14054574138] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb004ba00
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x5
[14056008351] [INFO] [task.user_enter] Entering user mode tid=10 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132621 RSP_BEFORE=18446744072368164448 RFLAGS_BEFORE=134 CR3_BEFORE=55140352 fs_base=0 gs_base=18446744071563780584
[14059624755] [INFO] [ps2_kbd] ps2_kbd: online (handle=5)
[14061592017] [INFO] [kernel::syscall::handlers::device] DEVICE: task subscribed to vector 0x21
[14062769556] [INFO] [ps2_kbd] ps2_kbd: subscribed to IRQ1 (vector 0x21)
[14063590530] [INFO] [ps2_kbd] ps2_kbd: entering interrupt-driven loop
[14069235576] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0095f70
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x7
[14070547227] [INFO] [task.user_enter] Entering user mode tid=11 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132621 RSP_BEFORE=18446744072368181856 RFLAGS_BEFORE=134 CR3_BEFORE=55234560 fs_base=0 gs_base=18446744071563780584
[14073813633] [INFO] [ps2_mouse] ps2_mouse: online (handle=7)
[14074561083] [INFO] [ps2_mouse] ps2_mouse: enabling aux port
[14078347206] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb00af0a8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600080009000b
[14079562629] [INFO] [task.user_enter] Entering user mode tid=12 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132621 RSP_BEFORE=18446744072368207728 RFLAGS_BEFORE=130 CR3_BEFORE=55336960 fs_base=0 gs_base=18446744071563780584
[14085978093] [INFO] [bristle] bristle: online (kbd=6, mouse=8, evt=9, evt_echo=11)
[14094082728] [INFO] [bristle] bristle: registered in graph as svc.Input (id=457)
[14102509806] [INFO] [sprout::pipelines] SPROUT: Bloom handles via BS=431 backend=BootFB
[14103794562] [INFO] [kernel::task::loader] Loading module: /boot/bloom
[14104359522] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[14105533266] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[14174485743] [INFO] [kernel::task::loader] Segment: vaddr=26e020 exec=false
[14175522306] [INFO] [kernel::task::loader]   Overlap at 26e000: merging perms to r=true w=false x=true
[14182666773] [INFO] [kernel::task::loader] Segment: vaddr=278d58 exec=false
[14183388186] [INFO] [kernel::task::loader]   Overlap at 278000: merging perms to r=true w=true x=true
[14190973863] [INFO] [sprout::pipelines] SPROUT: Spawned bloom (PID=13)
[14192902878] [INFO] [kernel::task::loader] Loading module: /boot/echo
[14193503874] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[14194685472] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[14197713585] [INFO] [kernel::task::loader] Segment: vaddr=202110 exec=false
[14198372925] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=false x=true
[14199896469] [INFO] [kernel::task::loader] Segment: vaddr=202e58 exec=false
[14200510995] [INFO] [kernel::task::loader]   Overlap at 202000: merging perms to r=true w=true x=true
[14206619724] [INFO] [sprout::pipelines] SPROUT: Spawned echo (PID=14)
[14207616126] [INFO] [sprout::pipelines] SPROUT: Input pipeline ready (keyboard + mouse)
[14208396180] [INFO] [sprout::supervisor] SPROUT: Entering supervisor loop.
[14221274595] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0014c40
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x1af
[14223119130] [INFO] [task.user_enter] Entering user mode tid=13 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132621 RSP_BEFORE=18446744072368250336 RFLAGS_BEFORE=130 CR3_BEFORE=55443456 fs_base=0 gs_base=18446744071563780584
[14226399462] [INFO] [bloom::logging] bloom: logging initialized
[14252040066] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb004ee60
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xc
[14253376335] [INFO] [task.user_enter] Entering user mode tid=14 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562132621 RSP_BEFORE=18446744072368267792 RFLAGS_BEFORE=130 CR3_BEFORE=56029184 fs_base=0 gs_base=18446744071563780584
[14256938949] [INFO] [echo] echo: online (handle=12)
[14257724844] [INFO] [echo] echo: ready for Bristle events (keyboard + mouse)
T:E580 [14292825360] [INFO] [ps2_mouse] ps2_mouse: controller cfg already correct (0x47)
[14293684944] [INFO] [ps2_mouse] ps2_mouse: sending enable command (0xF4)
[14294691147] [INFO] [ps2_mouse] ps2_mouse: enable ACK received (0xFA)
T:E410 T:D530 [14404051695] [INFO] [bloom::asset] [asset_bank] worker spawned tid=18 (priority=2)
T:25D0 [14409027039] [INFO] [bloom::asset] [asset_bank] worker started (priority bump)
[14410711590] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 179 (23272 bytes)
[14416950702] [INFO] [bloom::asset] [asset_bank] mapped at 0x10386000
[14417891862] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/DSEG7Classic-Regular.ttf'...
[14443779834] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[14447106795] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'DSEG7Classic-Regular.ttf' in slot 0
[14457299934] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 182 (309408 bytes)
[14463945606] [INFO] [bloom::asset] [asset_bank] mapped at 0x1038c000
[14465150865] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/Hack-Regular.ttf'...
[15529633878] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[15530951601] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'Hack-Regular.ttf' in slot 1
[15733527678] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 185 (569208 bytes)
[15743193807] [INFO] [bloom::asset] [asset_bank] mapped at 0x103d8000
[15744296733] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/NotoSans-Regular.ttf'...
[17690773353] [INFO] [bloom::compositor] bloom: compositor bytespace 376 (1280x720 stride=5120 format=1)
[18674096958] [INFO] [bloom::compositor] bloom: display backend: BootFB
[18678733128] [INFO] [ps2_mouse] ps2_mouse: init done
[18679412598] [INFO] [kernel::syscall::handlers::device] DEVICE: task subscribed to vector 0x2c
[18680294127] [INFO] [ps2_mouse] ps2_mouse: subscribed to IRQ12 (vector 0x2c)
[18680979207] [INFO] [ps2_mouse] ps2_mouse: entering interrupt-driven loop
[19327606683] [INFO] [bloom::compositor] bloom: mapped size=3686400 (source=bytespace_info)
[19436763456] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[19438021482] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'NotoSans-Regular.ttf' in slot 2
[19668464772] [INFO] [bloom::frame_loop] bloom: running (fps_target=60)
[19674871392] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 188 (258156 bytes)
[19678525185] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x100c1b90
[19679581152] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[19680369522] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[19681327644] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: NO FILTER (filter_ptr=0x0 filter_len=0)
[19687722582] [INFO] [stem::ui] UiBuilder: created root 513
[19697014029] [INFO] [display_bootfb] display_bootfb: bound bytespace 376
[19700422566] [INFO] [bloom::asset] [asset_bank] mapped at 0x10b6b000
[19701164175] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/NotoSansSymbol-Regular.ttf'...
[20020337634] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7fd610
[20021226819] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[20021993574] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[20022954699] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=489 subj_lo=0
[21557834562] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[21559217757] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'NotoSansSymbol-Regular.ttf' in slot 3
[21560158554] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 191 (656852 bytes)
[21572339712] [INFO] [clock] Found UI Root: 513 (attempt 5)
[21576653241] [INFO] [bloom::asset] [asset_bank] mapped at 0x10bab000
[21577854672] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/NotoSansSymbol2-Regular.ttf'...
[25259214486] [INFO] [bloom::asset] [asset_bank] SUCCESS: font parsed
[25260862638] [INFO] [bloom::asset] [asset_bank] publish_font (pending): 'NotoSansSymbol2-Regular.ttf' in slot 4
[25262605335] [INFO] [bloom::asset] [asset_bank] mapping font bytespace 194 (616196 bytes)
[25274376534] [INFO] [bloom::asset] [asset_bank] mapped at 0x10c4c000
[25275373563] [INFO] [bloom::asset] [asset_bank] parsing font '/assets/fonts/NotoSerif-Regular.ttf'...
[26280229008] [INFO] [bloom::reclaimer] [reclaimer] +102400 bytes (total: 102400)
[26281228083] [INFO] [bloom::asset] [asset_bank] promoting font 'DSEG7Classic-Regular.ttf' to gen=1 (102400b) in slot 0
[26282303916] [INFO] [bloom::reclaimer] [reclaimer] +102400 bytes (total: 204800)
[26282973255] [INFO] [bloom::asset] [asset_bank] promoting font 'Hack-Regular.ttf' to gen=1 (102400b) in slot 1
[26283772647] [INFO] [bloom::reclaimer] [reclaimer] +102400 bytes (total: 307200)
[26284397667] [INFO] [bloom::asset] [asset_bank] promoting font 'NotoSans-Regular.ttf' to gen=1 (102400b) in slot 2
[26286063903] [INFO] [bloom::reclaimer] [reclaimer] +102400 bytes (total: 409600)
[26286999684] [INFO] [bloom::asset] [asset_bank] promoting font 'NotoSansSymbol-Regular.ttf' to gen=1 (102400b) in slot 3
[26287884546] [INFO] [bloom::reclaimer] [reclaimer] +102400 bytes (total: 512000)
[26288535009] [INFO] [bloom::asset] [asset_bank] promoting font 'NotoSansSymbol2-Regular.ttf' to gen=1 (102400b) in slot 4
[39887767194] [INFO] [cambium] Found 1 bindings
[41198616558] [INFO] [clock] Binding created: 568 (source=356 target=552)
[41199679752] [INFO] [clock] CLOCK: Entering main loop, publishing to thing_id=356
[41202613650] [INFO] [clock] unix=1768966406 utc=2026-01-21 03:33:26 mono_ns=20600451613
[42870275283] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: ptr=0x7fedd0
[42871365339] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: validating range len=48
[42872246703] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: copyin success. mode=1 start_seq=0
[42873164037] [INFO] [kernel::syscall::handlers::root_handlers] sys_root_watch_open: DECODED FILTER: flags=0x4 kind=0 pred=0 subj_lo=356
[43854008385] [INFO] [cambium] Opened watch 583 for source 356 (binding 568, start_seq=0)
[43879550154] [INFO] [cambium] CATCH-UP: Draining 1 watches for historical events...
[45166044627] [INFO] [clock] CLOCK PUBLISH: thing=356 now_text='03:33:26' tick=20600451613

```
</details>
