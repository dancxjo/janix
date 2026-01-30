# ✅ Scenario: Designer checks time while browsing fonts

> Last run: 2026-01-29 22:19:33

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is running | ✅ | 9558ms | <a href="./01/after.png"><img src="./01/after.png" width="150" /></a> - [💾](./01/registers.txt) |
| 2 | And I wait for the system to reach ready state | ⏭️ | 206ms | - [📜](./02/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[18667540005] [CONTRACT] [kernel] thing-os kernel starting...
[18688929687] [INFO] [kernel::memory] Memory map has 64 entries
[18697262336] [INFO] [kernel::memory]   [0] 0x0 - 0xa0000 (Usable)
[18703202094] [INFO] [kernel::memory]   [1] 0x100000 - 0x800000 (Usable)
[18708653203] [INFO] [kernel::memory]   [2] 0x800000 - 0x808000 (Other)
[18714101641] [INFO] [kernel::memory]   [3] 0x808000 - 0x80b000 (Usable)
[18719591605] [INFO] [kernel::memory]   [4] 0x80b000 - 0x80c000 (Other)
[18752154473] [INFO] [kernel::memory]   [5] 0x80c000 - 0x811000 (Usable)
[18757545310] [INFO] [kernel::memory]   [6] 0x811000 - 0x900000 (Other)
[18763251107] [INFO] [kernel::memory]   [7] 0x900000 - 0x1780000 (Reserved)
[18769059630] [INFO] [kernel::memory]   [8] 0x1780000 - 0x78285000 (Usable)
[18774933958] [INFO] [kernel::memory]   [9] 0x78285000 - 0x782e9000 (Reserved)
[18780757658] [INFO] [kernel::memory]   [10] 0x782e9000 - 0x782ea000 (Other)
[18786350736] [INFO] [kernel::memory]   [11] 0x782ea000 - 0x782eb000 (Reserved)
[18792163593] [INFO] [kernel::memory]   [12] 0x782eb000 - 0x782ec000 (Other)
[18797892608] [INFO] [kernel::memory]   [13] 0x782ec000 - 0x782ed000 (Reserved)
[18803723941] [INFO] [kernel::memory]   [14] 0x782ed000 - 0x782ee000 (Other)
[18809456807] [INFO] [kernel::memory]   [15] 0x782ee000 - 0x782ef000 (Reserved)
[18815391626] [INFO] [kernel::memory]   [16] 0x782ef000 - 0x782f0000 (Other)
[18821181429] [INFO] [kernel::memory]   [17] 0x782f0000 - 0x782f1000 (Reserved)
[18827275666] [INFO] [kernel::memory]   [18] 0x782f1000 - 0x782f2000 (Other)
[18832845150] [INFO] [kernel::memory]   [19] 0x782f2000 - 0x782f3000 (Reserved)
[18838652695] [INFO] [kernel::memory]   [20] 0x782f3000 - 0x782f4000 (Other)
[18844342320] [INFO] [kernel::memory]   [21] 0x782f4000 - 0x782f5000 (Reserved)
[18850141993] [INFO] [kernel::memory]   [22] 0x782f5000 - 0x782f6000 (Other)
[18855802744] [INFO] [kernel::memory]   [23] 0x782f6000 - 0x782f7000 (Reserved)
[18861538547] [INFO] [kernel::memory]   [24] 0x782f7000 - 0x782f8000 (Other)
[18867229879] [INFO] [kernel::memory]   [25] 0x782f8000 - 0x782f9000 (Reserved)
[18873127628] [INFO] [kernel::memory]   [26] 0x782f9000 - 0x782fa000 (Other)
[18878642438] [INFO] [kernel::memory]   [27] 0x782fa000 - 0x782fb000 (Reserved)
[18884443940] [INFO] [kernel::memory]   [28] 0x782fb000 - 0x782fc000 (Other)
[18890457151] [INFO] [kernel::memory]   [29] 0x782fc000 - 0x782fd000 (Reserved)
[18896269308] [INFO] [kernel::memory]   [30] 0x782fd000 - 0x782fe000 (Other)
[18901924359] [INFO] [kernel::memory]   [31] 0x782fe000 - 0x782ff000 (Reserved)
[18907736227] [INFO] [kernel::memory]   [32] 0x782ff000 - 0x78300000 (Other)
[18913406535] [INFO] [kernel::memory]   [33] 0x78300000 - 0x78301000 (Reserved)
[18919349884] [INFO] [kernel::memory]   [34] 0x78301000 - 0x78302000 (Other)
[18924909022] [INFO] [kernel::memory]   [35] 0x78302000 - 0x78303000 (Reserved)
[18930850632] [INFO] [kernel::memory]   [36] 0x78303000 - 0x78304000 (Other)
[18936427728] [INFO] [kernel::memory]   [37] 0x78304000 - 0x78305000 (Reserved)
[18942247144] [INFO] [kernel::memory]   [38] 0x78305000 - 0x78306000 (Other)
[18948117398] [INFO] [kernel::memory]   [39] 0x78306000 - 0x78307000 (Reserved)
[18953928776] [INFO] [kernel::memory]   [40] 0x78307000 - 0x78308000 (Other)
[18960053887] [INFO] [kernel::memory]   [41] 0x78308000 - 0x78309000 (Reserved)
[18966471924] [INFO] [kernel::memory]   [42] 0x78309000 - 0x7830a000 (Other)
[18972545844] [INFO] [kernel::memory]   [43] 0x7830a000 - 0x7830b000 (Reserved)
[18978711991] [INFO] [kernel::memory]   [44] 0x7830b000 - 0x7830c000 (Other)
[18984773067] [INFO] [kernel::memory]   [45] 0x7830c000 - 0x7830d000 (Reserved)
[18990974271] [INFO] [kernel::memory]   [46] 0x7830d000 - 0x7830e000 (Other)
[18996924951] [INFO] [kernel::memory]   [47] 0x7830e000 - 0x7830f000 (Reserved)
[19003385743] [INFO] [kernel::memory]   [48] 0x7830f000 - 0x78310000 (Other)
[19010076139] [INFO] [kernel::memory]   [49] 0x78310000 - 0x78311000 (Reserved)
[19016934328] [INFO] [kernel::memory]   [50] 0x78311000 - 0x78312000 (Other)
[19023235611] [INFO] [kernel::memory]   [51] 0x78312000 - 0x78313000 (Reserved)
[19029344713] [INFO] [kernel::memory]   [52] 0x78313000 - 0x78314000 (Other)
[19035718232] [INFO] [kernel::memory]   [53] 0x78314000 - 0x78315000 (Reserved)
[19041785631] [INFO] [kernel::memory]   [54] 0x78315000 - 0x78316000 (Other)
[19047580108] [INFO] [kernel::memory]   [55] 0x78316000 - 0x78317000 (Reserved)
[19053433387] [INFO] [kernel::memory]   [56] 0x78317000 - 0x78318000 (Other)
[19059222207] [INFO] [kernel::memory]   [57] 0x78318000 - 0x78319000 (Reserved)
[19065191264] [INFO] [kernel::memory]   [58] 0x78319000 - 0x7831a000 (Other)
[19070869610] [INFO] [kernel::memory]   [59] 0x7831a000 - 0x7831b000 (Reserved)
[19077085506] [INFO] [kernel::memory]   [60] 0x7831b000 - 0x7831c000 (Other)
[19082914949] [INFO] [kernel::memory]   [61] 0x7831c000 - 0x7831d000 (Reserved)
[19088688755] [INFO] [kernel::memory]   [62] 0x7831d000 - 0x7831e000 (Other)
[19094449666] [INFO] [kernel::memory]   [63] 0x7831e000 - 0x7831f000 (Reserved)
[19100538242] [INFO] [kernel::memory] HHDM Offset: 0xffff800000000000
[19390368141] [CONTRACT] [kernel::memory] Frame allocator initialized with 488093 free frames
[19405462511] [INFO] [bran::arch] IOAPIC: hhdm=0xffff800000000000
[19421617688] [INFO] [bran::arch] IOAPIC: Disabling legacy PIC...
[19428426011] [INFO] [bran::arch] IOAPIC: PIC disabled OK
[19433731396] [INFO] [bran::arch] IOAPIC: RSDP virt=0x7f77e014
[19448335412] [INFO] [bran::arch] IOAPIC: MADT parsed OK
[19453688103] [INFO] [bran::arch] IOAPIC: Found at phys 0xfec00000, GSI base 0
[19463327546] [INFO] [bran::arch] IOAPIC: Registers initialized
[19469994316] [INFO] [bran::arch] IOAPIC: version 0x20, 24 redir entries
[19477173563] [INFO] [bran::arch] IOAPIC: All pins masked
[19484056537] [INFO] [bran::arch] IOAPIC: IRQ1 -> GSI 1 -> 0x21
[19489688766] [INFO] [bran::arch] IOAPIC: IRQ12 -> GSI 12 -> 0x2C
[19494857668] [INFO] [bran::arch] IOAPIC: Init complete
[19499914508] [CONTRACT] [kernel] Initializing global allocator...
[19931019082] [INFO] [kernel::memory::global_alloc] Global allocator initialized (LinkedHeap, 32MB)
[19939105682] [CONTRACT] [kernel] Initializing SIMD...
[19945175138] [CONTRACT] [kernel] Initializing tasking...
[19961467987] [INFO] [kernel::task::scheduler]   Acquiring scheduler lock...
[19969502656] [INFO] [kernel::task::scheduler]   Lock acquired, checking if initialized...
[19976817496] [INFO] [kernel::task::scheduler]   Allocating scheduler...
[19989449076] [INFO] [kernel::task::scheduler]   Leaking scheduler...
[19995088497] [INFO] [kernel::task::scheduler]   Initializing boot task...
[20002811642] [INFO] [kernel::task::scheduler]   Creating boot task...
[20013717897] [INFO] [kernel::task::scheduler]   Creating idle task...
[20025363034] [INFO] [kernel::task::scheduler]   Boot task initialized
[20031033435] [INFO] [kernel::task::scheduler]   Storing scheduler pointer...
[20037576276] [CONTRACT] [kernel::task::scheduler] Scheduler initialized
[20050136027] [INFO] [kernel::root] Spawning Root service...
[20064681122] [INFO] [kernel::root::boot_register] ROOT: boot registration begin (Census Phase 1 v0.2)
[20084144982] [INFO] [kernel::root::service] ROOT: started once
[24120788089] [INFO] [kernel::root::boot_register] ROOT: Census Phase 2: PCI
[24127767886] [INFO] [kernel::root::pci] PCI: Starting enumeration...
[24194547454] [INFO] [kernel::root::pci] PCI: 00:00.0 8086:29c0 Intel Corporation 82G33/G31/P35/P31 Express DRAM Controller class=06:00 prog_if=00 rev=00
[24232638009] [INFO] [kernel::root::pci] PCI: 00:01.0 1234:1111 (unknown vendor) (unknown device) class=03:00 prog_if=00 rev=02
[24286176514] [INFO] [kernel::root::pci] PCI: 00:02.0 8086:10d3 Intel Corporation 82574L Gigabit Network Connection class=02:00 prog_if=00 rev=00
[24350439557] [INFO] [kernel::root::pci] PCI: 00:1f.0 8086:2918 Intel Corporation 82801IB (ICH9) LPC Interface Controller class=06:01 prog_if=00 rev=02
[24362739577] [INFO] [kernel::root::pci] PCI: Found LPC/ISA bridge at 00:1f.0
[24438667656] [INFO] [kernel::root::pci] LPC: Created Legacy IO bus with CMOS and PS/2 controller
[24488302155] [INFO] [kernel::root::pci] PCI: 00:1f.2 8086:2922 Intel Corporation 82801IR/IO/IH (ICH9R/DO/DH) 6 port SATA Controller [AHCI mode] class=01:06 prog_if=01 rev=02
[24509803498] [INFO] [kernel::root::pci] PCI: Found AHCI SATA controller at 00:1f.2
[24519392400] [INFO] [kernel::root::pci] PCI: Registered AHCI controller (graph_id=656, idx=3) BAR5=0x810c4000
[24568993942] [INFO] [kernel::root::pci] PCI: 00:1f.3 8086:2930 Intel Corporation 82801I (ICH9 Family) SMBus Controller class=0c:05 prog_if=00 rev=02
[24582857667] [INFO] [kernel::root::boot_register] ROOT: registered items. host=3 kernel=6
[24591074022] [CONTRACT] [kernel] KERNEL: root census complete: host=t3 kernel=t6 root=t7
[24601335344] [INFO] [kernel] Found init module: /boot/sprout (cmdline: ''), loading...
[24611737047] [INFO] [kernel::task::loader] Loading module: /boot/sprout
[24618299058] [INFO] [kernel::task::loader]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[24637835024] [INFO] [kernel::task::loader] Segment: vaddr=200000 exec=true
[24660751516] [INFO] [kernel::task::loader] Segment: vaddr=210000 exec=false
[24671360552] [INFO] [kernel::task::loader] Segment: vaddr=215000 exec=false
[24687443537] [INFO] [kernel] Warning: Module registry page overflow, truncating list.
[24695368343] [INFO] [kernel] Spawning sprout with registry at 0x600000...
[24701636183] [CONTRACT] [kernel] Spawning init process...
[24708777050] [INFO] [kernel] System initialized. Setting up preemption timer (100Hz)...
[24740442559] [INFO] [bran::arch::x86_64::ioapic] LAPIC: calibrated timer (62076200 ticks/sec), init_cnt=620762 for 100Hz
[24751045138] [CONTRACT] [kernel] Entering scheduler loop.
[24755975901] [INFO] [kernel::tests::time_test] TIME ANCHORING TEST: Starting...
[24763566294] [INFO] [kernel::tests::time_test] TIME ANCHORING TEST: FAIL - sys_time_now returned 12381533231 but should have failed before anchor
[24774517209] [INFO] [kernel::tests::time_monotonic_test] TIME MONOTONIC TEST: Starting...
[24785580508] [INFO] [kernel::tests::time_monotonic_test] TIME MONOTONIC TEST: 1000 samples monotonic - PASS
[24794826864] [INFO] [kernel::tests::time_monotonic_test] TIME MONOTONIC TEST: trace::now() monotonic - PASS
[24804060198] [INFO] [kernel::tests::time_monotonic_test] TIME MONOTONIC TEST: trace and syscall consistent - PASS
[24813012345] [INFO] [kernel::tests::time_monotonic_test] TIME MONOTONIC TEST: All tests PASS
[24820531810] [INFO] [kernel::tests::fw_tables] FW_TABLES SELFTEST: Starting...
[24828011178] [INFO] [kernel::tests::fw_tables] FW_TABLES SELFTEST: PASS
[24858412241] [INFO] [kernel::task::scheduler::spawn] Trampoline entered. Arg: 0xffffffffb0008580
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600000
[24876065429] [INFO] [task.user_enter] Entering user mode tid=3 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562136589 RSP_BEFORE=18446744072367741888 RFLAGS_BEFORE=134 CR3_BEFORE=50331648 fs_base=0 gs_base=18446744071563860640

```
</details>
