# ✅ Scenario: Server starts up

> Last run: 2026-02-11 17:01:27

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | When I start the machine | ✅ | 8670ms | <a href="./01/after.png"><img src="./01/after.png" width="150" /></a> [📜](./01/serial.log) [💾](./01/registers.txt) |
| 2 | Then I should see a message in the serial output that says "anther: Listening on port 80" | ✅ | 2139ms | <a href="./02/after.png"><img src="./02/after.png" width="150" /></a> [📜](./02/serial.log) [💾](./02/registers.txt) |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[16534989992] [INFO] [kernel] [CPU0] BOOTFB: width=1920 height=1080 pitch=7680 bpp=32 format=Bgrx8888
[16541560967] [CONTRACT] [kernel] [CPU0] thing-os kernel starting...
[16545504941] [INFO] [bran::requests] [CPU0] Limine: Found 35 boot modules
[16547585139] [INFO] [bran::requests] [CPU0]   [0] /boot/sprout (cmdline='init') size=107024
[16549551503] [INFO] [bran::requests] [CPU0]   [1] /boot/bristle (cmdline='init') size=33200
[16550096613] [INFO] [bran::requests] [CPU0]   [2] /boot/rtc_cmos (cmdline='init') size=33368
[16550597678] [INFO] [bran::requests] [CPU0]   [3] /boot/clock (cmdline='') size=66064
[16551100801] [INFO] [bran::requests] [CPU0]   [4] /boot/taskman (cmdline='') size=74256
[16551510214] [INFO] [bran::requests] [CPU0]   [5] /boot/ps2_kbd (cmdline='init') size=25008
[16551908880] [INFO] [bran::requests] [CPU0]   [6] /boot/echo (cmdline='init') size=29104
[16552364714] [INFO] [bran::requests] [CPU0]   [7] /boot/bloom (cmdline='init') size=948744
[16552776038] [INFO] [bran::requests] [CPU0]   [8] /boot/ps2_mouse (cmdline='init') size=25008
[16553428942] [INFO] [bran::requests] [CPU0]   [9] /boot/display_bootfb (cmdline='init') size=33368
[16553950358] [INFO] [bran::requests] [CPU0]   [10] /boot/display_virtio_gpu (cmdline='init') size=53864
[16554430779] [INFO] [bran::requests] [CPU0]   [11] /boot/fontd (cmdline='init') size=188944
[16554868092] [INFO] [bran::requests] [CPU0]   [12] /boot/blossom (cmdline='init') size=119312
[16555424298] [INFO] [bran::requests] [CPU0]   [13] /boot/flytrap (cmdline='') size=287256
[16555832757] [INFO] [bran::requests] [CPU0]   [14] /boot/virtio_netd (cmdline='') size=49680
[16556248291] [INFO] [bran::requests] [CPU0]   [15] /boot/rtl8168d (cmdline='') size=45584
[16556660459] [INFO] [bran::requests] [CPU0]   [16] /boot/netd (cmdline='') size=147984
[16557063957] [INFO] [bran::requests] [CPU0]   [17] /boot/fetchd (cmdline='') size=49680
[16557761908] [INFO] [bran::requests] [CPU0]   [18] /boot/anther (cmdline='') size=602672
[16558207404] [INFO] [bran::requests] [CPU0]   [19] /boot/photosynthesis (cmdline='') size=12648
[16558656605] [INFO] [bran::requests] [CPU0]   [20] /boot/ahci_disk (cmdline='') size=45664
[16559077308] [INFO] [bran::requests] [CPU0]   [21] /boot/iso9660d (cmdline='') size=49760
[16559509701] [INFO] [bran::requests] [CPU0]   [22] /boot/virtio_sound (cmdline='init') size=45488
[16560190688] [INFO] [bran::requests] [CPU0]   [23] /boot/hdaudio (cmdline='') size=33200
[16560741701] [INFO] [bran::requests] [CPU0]   [24] /boot/pci_stubd (cmdline='') size=29104
[16561211334] [INFO] [bran::requests] [CPU0]   [25] /boot/beeper (cmdline='init') size=33296
[16561708524] [INFO] [bran::requests] [CPU0]   [26] /boot/nectar (cmdline='init') size=176656
[16562137672] [INFO] [bran::requests] [CPU0]   [27] /assets/wallpapers/clouds.bmp (cmdline='') size=786486
[16562686730] [INFO] [bran::requests] [CPU0]   [28] /assets/wallpapers/flower.bmp (cmdline='') size=8386614
[16563201942] [INFO] [bran::requests] [CPU0]   [29] /assets/wallpapers/leather.bmp (cmdline='') size=1179702
[16563679005] [INFO] [bran::requests] [CPU0]   [30] /assets/wallpapers/linen.bmp (cmdline='') size=4718646
[16564159114] [INFO] [bran::requests] [CPU0]   [31] /assets/fonts/NotoSans-Regular.ttf (cmdline='') size=569208
[16564722208] [INFO] [bran::requests] [CPU0]   [32] /assets/themes/genie_circles.wasm (cmdline='') size=2824
[16565203018] [INFO] [bran::requests] [CPU0]   [33] /assets/cursors/future/default.svg (cmdline='') size=3051
[16565682864] [INFO] [bran::requests] [CPU0]   [34] /boot/locale.conf (cmdline='') size=93
[16567243937] [INFO] [kernel::memory] [CPU0] Memory map has 64 entries
[16568624103] [INFO] [kernel::memory] [CPU0]   [0] 0x0 - 0xa0000 (Usable)
[16569449406] [INFO] [kernel::memory] [CPU0]   [1] 0x100000 - 0x800000 (Usable)
[16569802717] [INFO] [kernel::memory] [CPU0]   [2] 0x800000 - 0x808000 (Other)
[16570177943] [INFO] [kernel::memory] [CPU0]   [3] 0x808000 - 0x80b000 (Usable)
[16570569484] [INFO] [kernel::memory] [CPU0]   [4] 0x80b000 - 0x80c000 (Other)
[16570947856] [INFO] [kernel::memory] [CPU0]   [5] 0x80c000 - 0x811000 (Usable)
[16571311539] [INFO] [kernel::memory] [CPU0]   [6] 0x811000 - 0x900000 (Other)
[16571744496] [INFO] [kernel::memory] [CPU0]   [7] 0x900000 - 0x1780000 (Reserved)
[16572161427] [INFO] [kernel::memory] [CPU0]   [8] 0x1780000 - 0x77d2f000 (Usable)
[16572548594] [INFO] [kernel::memory] [CPU0]   [9] 0x77d2f000 - 0x77d90000 (Reserved)
[16573387080] [INFO] [kernel::memory] [CPU0]   [10] 0x77d90000 - 0x77d91000 (Other)
[16573896930] [INFO] [kernel::memory] [CPU0]   [11] 0x77d91000 - 0x77d92000 (Reserved)
[16574280560] [INFO] [kernel::memory] [CPU0]   [12] 0x77d92000 - 0x77d93000 (Other)
[16574647380] [INFO] [kernel::memory] [CPU0]   [13] 0x77d93000 - 0x77d94000 (Reserved)
[16575025862] [INFO] [kernel::memory] [CPU0]   [14] 0x77d94000 - 0x77d95000 (Other)
[16575390916] [INFO] [kernel::memory] [CPU0]   [15] 0x77d95000 - 0x77d96000 (Reserved)
[16575880905] [INFO] [kernel::memory] [CPU0]   [16] 0x77d96000 - 0x77e21000 (Other)
[16576362851] [INFO] [kernel::memory] [CPU0]   [17] 0x77e21000 - 0x77e22000 (Reserved)
[16576749109] [INFO] [kernel::memory] [CPU0]   [18] 0x77e22000 - 0x782a3000 (Other)
[16577116348] [INFO] [kernel::memory] [CPU0]   [19] 0x782a3000 - 0x782a4000 (Reserved)
[16577495031] [INFO] [kernel::memory] [CPU0]   [20] 0x782a4000 - 0x783c5000 (Other)
[16577902431] [INFO] [kernel::memory] [CPU0]   [21] 0x783c5000 - 0x783c6000 (Reserved)
[16578413176] [INFO] [kernel::memory] [CPU0]   [22] 0x783c6000 - 0x78bc6000 (Other)
[16578794895] [INFO] [kernel::memory] [CPU0]   [23] 0x78bc6000 - 0x78bc7000 (Reserved)
[16579175070] [INFO] [kernel::memory] [CPU0]   [24] 0x78bc7000 - 0x78c88000 (Other)
[16579553877] [INFO] [kernel::memory] [CPU0]   [25] 0x78c88000 - 0x78c89000 (Reserved)
[16579932840] [INFO] [kernel::memory] [CPU0]   [26] 0x78c89000 - 0x78cb5000 (Other)
[16580299642] [INFO] [kernel::memory] [CPU0]   [27] 0x78cb5000 - 0x78cb7000 (Reserved)
[16580755797] [INFO] [kernel::memory] [CPU0]   [28] 0x78cb7000 - 0x78cc0000 (Other)
[16581128765] [INFO] [kernel::memory] [CPU0]   [29] 0x78cc0000 - 0x78cc2000 (Reserved)
[16581508314] [INFO] [kernel::memory] [CPU0]   [30] 0x78cc2000 - 0x78cca000 (Other)
[16581874881] [INFO] [kernel::memory] [CPU0]   [31] 0x78cca000 - 0x78ccb000 (Reserved)
[16582253477] [INFO] [kernel::memory] [CPU0]   [32] 0x78ccb000 - 0x78cd4000 (Other)
[16582637831] [INFO] [kernel::memory] [CPU0]   [33] 0x78cd4000 - 0x78cd5000 (Reserved)
[16583097602] [INFO] [kernel::memory] [CPU0]   [34] 0x78cd5000 - 0x78ce1000 (Other)
[16583466156] [INFO] [kernel::memory] [CPU0]   [35] 0x78ce1000 - 0x78ce3000 (Reserved)
[16583842299] [INFO] [kernel::memory] [CPU0]   [36] 0x78ce3000 - 0x78cf0000 (Other)
[16584208174] [INFO] [kernel::memory] [CPU0]   [37] 0x78cf0000 - 0x78cf1000 (Reserved)
[16584585012] [INFO] [kernel::memory] [CPU0]   [38] 0x78cf1000 - 0x78cfd000 (Other)
[16585082533] [INFO] [kernel::memory] [CPU0]   [39] 0x78cfd000 - 0x78cfe000 (Reserved)
[16585529119] [INFO] [kernel::memory] [CPU0]   [40] 0x78cfe000 - 0x78d02000 (Other)
[16585887142] [INFO] [kernel::memory] [CPU0]   [41] 0x78d02000 - 0x78d03000 (Reserved)
[16586251738] [INFO] [kernel::memory] [CPU0]   [42] 0x78d03000 - 0x78d97000 (Other)
[16586671220] [INFO] [kernel::memory] [CPU0]   [43] 0x78d97000 - 0x78d98000 (Reserved)
[16587063350] [INFO] [kernel::memory] [CPU0]   [44] 0x78d98000 - 0x78da5000 (Other)
[16587510814] [INFO] [kernel::memory] [CPU0]   [45] 0x78da5000 - 0x78da6000 (Reserved)
[16587923797] [INFO] [kernel::memory] [CPU0]   [46] 0x78da6000 - 0x78dcb000 (Other)
[16588295615] [INFO] [kernel::memory] [CPU0]   [47] 0x78dcb000 - 0x78dcc000 (Reserved)
[16588675206] [INFO] [kernel::memory] [CPU0]   [48] 0x78dcc000 - 0x78dd8000 (Other)
[16589058350] [INFO] [kernel::memory] [CPU0]   [49] 0x78dd8000 - 0x78dd9000 (Reserved)
[16589486452] [INFO] [kernel::memory] [CPU0]   [50] 0x78dd9000 - 0x78de6000 (Other)
[16589966218] [INFO] [kernel::memory] [CPU0]   [51] 0x78de6000 - 0x78de7000 (Reserved)
[16590411118] [INFO] [kernel::memory] [CPU0]   [52] 0x78de7000 - 0x78e2e000 (Other)
[16590865514] [INFO] [kernel::memory] [CPU0]   [53] 0x78e2e000 - 0x78e2f000 (Reserved)
[16591256490] [INFO] [kernel::memory] [CPU0]   [54] 0x78e2f000 - 0x78e4d000 (Other)
[16591650793] [INFO] [kernel::memory] [CPU0]   [55] 0x78e4d000 - 0x78e4f000 (Reserved)
[16592192066] [INFO] [kernel::memory] [CPU0]   [56] 0x78e4f000 - 0x78e7e000 (Other)
[16592561719] [INFO] [kernel::memory] [CPU0]   [57] 0x78e7e000 - 0x78e80000 (Reserved)
[16592943672] [INFO] [kernel::memory] [CPU0]   [58] 0x78e80000 - 0x78e8e000 (Other)
[16593309437] [INFO] [kernel::memory] [CPU0]   [59] 0x78e8e000 - 0x78e90000 (Reserved)
[16593686098] [INFO] [kernel::memory] [CPU0]   [60] 0x78e90000 - 0x78e99000 (Other)
[16594052695] [INFO] [kernel::memory] [CPU0]   [61] 0x78e99000 - 0x78e9b000 (Reserved)
[16594641416] [INFO] [kernel::memory] [CPU0]   [62] 0x78e9b000 - 0x78ea2000 (Other)
[16595013020] [INFO] [kernel::memory] [CPU0]   [63] 0x78ea2000 - 0x78f8a000 (Other)
[16595631474] [INFO] [kernel::memory] [CPU0] HHDM Offset: 0xffff800000000000
[16872561786] [CONTRACT] [kernel::memory] [CPU0] Frame allocator initialized with 486728 free frames
[16884973536] [INFO] [bran::arch] [CPU0] IOAPIC: hhdm=0xffff800000000000
[16891809469] [INFO] [bran::arch] [CPU0] IOAPIC: Disabling legacy PIC...
[16893189545] [INFO] [bran::arch] [CPU0] IOAPIC: PIC disabled OK
[16894174044] [INFO] [bran::arch] [CPU0] IOAPIC: RSDP virt=0x7f77e014
[16899326681] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: total length 120
[16901241821] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77802c
[16902351106] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 1, len 12 at 0xffff80007f778034
[16903365208] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778040
[16904261439] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f77804a
[16904938696] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778054
[16905398549] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f77805e
[16905838576] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778068
[16906279927] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 4, len 6 at 0xffff80007f778072
[16907361611] [INFO] [bran::arch] [CPU0] IOAPIC: MADT parsed OK
[16908407054] [INFO] [bran::arch] [CPU0] SMP: Found 1 CPUs (CPU_COUNT now = 1)
[16909091999] [INFO] [bran::arch] [CPU0] IOAPIC: Found at phys 0xfec00000, GSI base 0
[16910863088] [INFO] [bran::arch] [CPU0] IOAPIC: Registers initialized
[16911841476] [INFO] [bran::arch] [CPU0] IOAPIC: Local APIC enabled (SVR=0x1FF, TPR=0)
[16912709230] [INFO] [bran::arch] [CPU0] IOAPIC: version 0x20, 24 redir entries
[16914635460] [INFO] [bran::arch] [CPU0] IOAPIC: All pins masked
[16916503848] [INFO] [bran::arch] [CPU0] IOAPIC: IRQ1 -> GSI 1 -> 0x21
[16917318289] [INFO] [bran::arch] [CPU0] IOAPIC: IRQ12 -> GSI 12 -> 0x2C
[16917826398] [INFO] [bran::arch] [CPU0] IOAPIC: Init complete
[16918508791] [CONTRACT] [kernel] [CPU0] Initializing global allocator...
[17415481264] [INFO] [kernel::memory::global_alloc] [CPU0] Global allocator initialized (LinkedHeap, 32MB)
[17416310483] [CONTRACT] [kernel] [CPU0] Initializing SIMD...
[17417980353] [CONTRACT] [kernel] [CPU0] Initializing tasking...
[17424381356] [INFO] [kernel::task::scheduler] [CPU0]   Acquiring scheduler lock...
[17426111555] [INFO] [kernel::task::scheduler] [CPU0]   Lock acquired, checking if initialized...
[17426881744] [INFO] [kernel::task::scheduler] [CPU0]   Allocating scheduler...
[17435227838] [INFO] [kernel::task::scheduler] [CPU0]   Leaking scheduler...
[17435794955] [INFO] [kernel::task::scheduler] [CPU0]   Initializing boot task...
[17439833406] [INFO] [kernel::task::scheduler] [CPU0]   Creating boot task...
[17450256839] [INFO] [kernel::task::scheduler] [CPU0]   Creating idle tasks...
[17452076422] [DEBUG] [kernel::task::scheduler::spawn] [CPU0] SCHED: Task 1 assigned to CPU 0
[17467330388] [INFO] [kernel::task::scheduler] [CPU0]   Creating graph worker task...
[17468689941] [DEBUG] [kernel::task::scheduler::spawn] [CPU0] SCHED: Task 2 assigned to CPU 0
[17470470512] [INFO] [kernel::task::scheduler] [CPU0]   Boot task initialized
[17471088359] [INFO] [kernel::task::scheduler] [CPU0]   Storing scheduler pointer...
[17472037824] [CONTRACT] [kernel::task::scheduler] [CPU0] Scheduler initialized
[17472920228] [INFO] [kernel] [CPU0] Kernel: Detected 1 CPU.
[17480543879] [INFO] [kernel::root] [CPU0] Spawning Root service...
[17481649181] [DEBUG] [kernel::task::scheduler::spawn] [CPU0] SCHED: Task 3 assigned to CPU 0
[17483148695] [INFO] [kernel::root::boot_register] [CPU0] ROOT: boot registration begin (Census Phase 1 v0.2)
[17483787287] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: registering Host...
[17498601702] [INFO] [kernel::root::service] [CPU0] ROOT: started once
[17499123949] [CONTRACT] [kernel::root::service] [CPU0] ROOT: Initializing components...
[17500238084] [CONTRACT] [kernel::root::service] [CPU0] ROOT: Graph initialized
[17501047706] [CONTRACT] [kernel::root::service] [CPU0] ROOT: Journal initialized
[17501687802] [CONTRACT] [kernel::root::service] [CPU0] ROOT: Interner initialized
[17512467812] [CONTRACT] [kernel::root::service] [CPU0] ROOT: LogSymbols initialized
[17516040929] [CONTRACT] [kernel::root::service] [CPU0] ROOT: BatchScratch initialized
[17517055622] [CONTRACT] [kernel::root::service] [CPU0] ROOT: QueryScratch initialized
[17517567573] [CONTRACT] [kernel::root::service] [CPU0] ROOT: Entering main loop
[17540781891] [INFO] [kernel::task::scheduler] [CPU0] PROF: sched 2s: graph_flush calls=0 items=0 avg_us=0 max_us=0 slow=0 trylock_miss=0 qlen=13 q_hwm=13 q_drop_state=0 q_evict=0
[17570972704] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: Host registered: t1
[17997051655] [CONTRACT] [alloc] [CPU0] MEDIUM ALLOC #1: 120 KB align=8 total=0MB
[18380543914] [INFO] [kernel::root::service] [CPU0] ROOT STATS: iter=1000 nodes=75 watches=0 history=496 journal=421 symbols=49 drops=0
[18409352508] [CONTRACT] [alloc] [CPU0] MEDIUM ALLOC #2: 240 KB align=8 total=0MB
[19142681917] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: Census Phase 2: PCI
[19143420106] [INFO] [kernel::root::pci] [CPU0] PCI: Starting enumeration...
[19251532369] [INFO] [kernel::root::service] [CPU0] ROOT STATS: iter=2000 nodes=186 watches=0 history=947 journal=756 symbols=134 drops=0
[19282877218] [INFO] [kernel::root::pci] [CPU0] PCI: 00:00.0 8086:29c0 Intel Corporation 82G33/G31/P35/P31 Express DRAM Controller class=06:00 prog_if=00 rev=00
[19314595274] [INFO] [kernel::root::pci] [CPU0] PCI: 00:01.0 1234:1111 (unknown vendor) (unknown device) class=03:00 prog_if=00 rev=02
[19324007326] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR0: phys=0x80000000 size=0x1000000
[19324720472] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR2: phys=0x81082000 size=0x1000
[19354796047] [INFO] [kernel::root::pci_stub] [CPU0] PCI: Registered PCI GPU stub (graph_id=187, idx=1)
[19393425717] [INFO] [kernel::root::pci] [CPU0] PCI: 00:02.0 1af4:1000 Red Hat, Inc. Virtio network device class=02:00 prog_if=00 rev=00
[19403754923] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR1: phys=0x81081000 size=0x1000
[19404363948] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR4: phys=0xc000000000 size=0x4000
[19409104382] [INFO] [kernel::root::pci] [CPU0] PCI: Found virtio network controller at 00:02.0
[19423918905] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO notify_cfg BAR4 offset=0x3000 mult=4
[19435758865] [INFO] [kernel::root::pci] [CPU0] PCI: VirtIO common_cfg BAR4 offset=0x0
[19439195132] [INFO] [kernel::root::pci] [CPU0] PCI: Registered virtio network (graph_id=189, idx=2) BAR4=0xc000000000
[19440044871] [INFO] [kernel::root::pci] [CPU0] VirtIO-Net: Device registered for userspace driver
[19485023431] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.0 8086:2918 Intel Corporation 82801IB (ICH9) LPC Interface Controller class=06:01 prog_if=00 rev=02
[19486454279] [INFO] [kernel::root::pci] [CPU0] PCI: Found LPC/ISA bridge at 00:1f.0
[19546160333] [INFO] [kernel::root::pci] [CPU0] LPC: Created Legacy IO bus with CMOS and PS/2 controller
[19585925246] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.2 8086:2922 Intel Corporation 82801IR/IO/IH (ICH9R/DO/DH) 6 port SATA Controller [AHCI mode] class=01:06 prog_if=01 rev=02
[19592042382] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR5: phys=0x81080000 size=0x1000
[19595027401] [INFO] [kernel::root::pci] [CPU0] PCI: Found AHCI SATA controller at 00:1f.2
[19597364882] [INFO] [kernel::root::pci] [CPU0] PCI: Registered AHCI controller (graph_id=198, idx=5) BAR5=0x81080000
[19636869953] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.3 8086:2930 Intel Corporation 82801I (ICH9 Family) SMBus Controller class=0c:05 prog_if=00 rev=02
[19639781080] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: registered items. host=t1 kernel=t3
[19640630492] [CONTRACT] [kernel] [CPU0] KERNEL: root census complete: host=t1 kernel=t3 root=t4
[19641320591] [CONTRACT] [kernel] [CPU0] Kernel: Enumerating 35 boot modules...
[19642158709] [CONTRACT] [kernel] [CPU0]   [0] name='/boot/sprout' cmdline='init' size=107024
[19642767922] [CONTRACT] [kernel] [CPU0]   [1] name='/boot/bristle' cmdline='init' size=33200
[19643194042] [CONTRACT] [kernel] [CPU0]   [2] name='/boot/rtc_cmos' cmdline='init' size=33368
[19643795093] [CONTRACT] [kernel] [CPU0]   [3] name='/boot/clock' cmdline='' size=66064
[19644243955] [CONTRACT] [kernel] [CPU0]   [4] name='/boot/taskman' cmdline='' size=74256
[19644626626] [CONTRACT] [kernel] [CPU0]   [5] name='/boot/ps2_kbd' cmdline='init' size=25008
[19645020557] [CONTRACT] [kernel] [CPU0]   [6] name='/boot/echo' cmdline='init' size=29104
[19645401098] [CONTRACT] [kernel] [CPU0]   [7] name='/boot/bloom' cmdline='init' size=948744
[19645805318] [CONTRACT] [kernel] [CPU0]   [8] name='/boot/ps2_mouse' cmdline='init' size=25008
[19646209710] [CONTRACT] [kernel] [CPU0]   [9] name='/boot/display_bootfb' cmdline='init' size=33368
[19646702293] [CONTRACT] [kernel] [CPU0]   [10] name='/boot/display_virtio_gpu' cmdline='init' size=53864
[19647149360] [CONTRACT] [kernel] [CPU0]   [11] name='/boot/fontd' cmdline='init' size=188944
[19647575397] [CONTRACT] [kernel] [CPU0]   [12] name='/boot/blossom' cmdline='init' size=119312
[19648039862] [CONTRACT] [kernel] [CPU0]   [13] name='/boot/flytrap' cmdline='' size=287256
[19648647198] [CONTRACT] [kernel] [CPU0]   [14] name='/boot/virtio_netd' cmdline='' size=49680
[19649067996] [CONTRACT] [kernel] [CPU0]   [15] name='/boot/rtl8168d' cmdline='' size=45584
[19649463729] [CONTRACT] [kernel] [CPU0]   [16] name='/boot/netd' cmdline='' size=147984
[19649848757] [CONTRACT] [kernel] [CPU0]   [17] name='/boot/fetchd' cmdline='' size=49680
[19650250009] [CONTRACT] [kernel] [CPU0]   [18] name='/boot/anther' cmdline='' size=602672
[19650668444] [CONTRACT] [kernel] [CPU0]   [19] name='/boot/photosynthesis' cmdline='' size=12648
[19651145625] [CONTRACT] [kernel] [CPU0]   [20] name='/boot/ahci_disk' cmdline='' size=45664
[19651551471] [CONTRACT] [kernel] [CPU0]   [21] name='/boot/iso9660d' cmdline='' size=49760
[19651909418] [CONTRACT] [kernel] [CPU0]   [22] name='/boot/virtio_sound' cmdline='init' size=45488
[19652296134] [CONTRACT] [kernel] [CPU0]   [23] name='/boot/hdaudio' cmdline='' size=33200
[19652664914] [CONTRACT] [kernel] [CPU0]   [24] name='/boot/pci_stubd' cmdline='' size=29104
[19653076404] [CONTRACT] [kernel] [CPU0]   [25] name='/boot/beeper' cmdline='init' size=33296
[19653545770] [CONTRACT] [kernel] [CPU0]   [26] name='/boot/nectar' cmdline='init' size=176656
[19653950156] [CONTRACT] [kernel] [CPU0]   [27] name='/assets/wallpapers/clouds.bmp' cmdline='' size=786486
[19654396983] [CONTRACT] [kernel] [CPU0]   [28] name='/assets/wallpapers/flower.bmp' cmdline='' size=8386614
[19654854778] [CONTRACT] [kernel] [CPU0]   [29] name='/assets/wallpapers/leather.bmp' cmdline='' size=1179702
[19655350521] [CONTRACT] [kernel] [CPU0]   [30] name='/assets/wallpapers/linen.bmp' cmdline='' size=4718646
[19655928767] [CONTRACT] [kernel] [CPU0]   [31] name='/assets/fonts/NotoSans-Regular.ttf' cmdline='' size=569208
[19656425681] [CONTRACT] [kernel] [CPU0]   [32] name='/assets/themes/genie_circles.wasm' cmdline='' size=2824
[19656894913] [CONTRACT] [kernel] [CPU0]   [33] name='/assets/cursors/future/default.svg' cmdline='' size=3051
[19657384750] [CONTRACT] [kernel] [CPU0]   [34] name='/boot/locale.conf' cmdline='' size=93
[19658889844] [INFO] [kernel] [CPU0] Found init module: /boot/sprout (cmdline: 'init'), loading...
[19661226019] [INFO] [kernel::task::loader] [CPU0] Loading module: /boot/sprout
[19662020122] [DEBUG] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[19670679552] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[19696859003] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=213000 exec=false
[19701748107] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=219000 exec=false
[19712925892] [INFO] [kernel] [CPU0] Spawning sprout with registry at 0x600000...
[19713572462] [CONTRACT] [kernel] [CPU0] Spawning init process...
[19715561386] [DEBUG] [kernel::task::scheduler::spawn] [CPU0] SCHED: Task 4 (user task/process) assigned to CPU 0
[19717404513] [INFO] [kernel] [CPU0] System initialized. Setting up preemption timer (100Hz)...
[19742561522] [INFO] [bran::arch::x86_64] [CPU0] LAPIC: calibrated timer (62428200 ticks/sec), init_cnt=624282 for 100Hz
[19743532516] [CONTRACT] [kernel] [CPU0] Entering scheduler loop.
[19748155809] [DEBUG] [kernel::task::scheduler::spawn] [CPU0] Trampoline entered. Arg: 0xffffffffb0024a20
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600000
[19753638647] [INFO] [bran::arch::x86_64::enter_user] [CPU0] Entering user mode tid=4 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562074941 RSP_BEFORE=18446744072368227888 RFLAGS_BEFORE=134 CR3_BEFORE=50327552 fs_base=0 gs_base=18446744071564225888
[19808068814] [INFO] [sprout] [CPU0] [sprout] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ff9b0 rip=0x2013ff rflags=0x202
[19809743790] [INFO] [sprout] [CPU0] SPROUT: v0.4 starting (Supervisor Mode)...
[19810554130] [INFO] [sprout::devtree] [CPU0] SPROUT: devtree::init entry (v0.2)
[19811326980] [INFO] [sprout::devtree] [CPU0] SPROUT: Step 1: Find Host
[19822302818] [INFO] [sprout::devtree] [CPU0] SPROUT: Step 2: HHDM
[19829564843] [INFO] [sprout::devtree] [CPU0] SPROUT: Step 3: Platform Bus
[19832838569] [INFO] [sprout::devtree] [CPU0] SPROUT: Step 4: Firmware
[19833817985] [INFO] [sprout::devtree] [CPU0] SPROUT: Finding ACPI...
[19836921210] [INFO] [sprout::devtree] [CPU0] SPROUT: Found 1 ACPI nodes
[19839606256] [INFO] [sprout::devtree] [CPU0] SPROUT: ACPI RSDP = 0x7f77e014
[19840405065] [INFO] [sprout::devtree] [CPU0] SPROUT: Finding DTB...
[19843698848] [INFO] [sprout::devtree] [CPU0] SPROUT: Found 0 DTB nodes
[19844606562] [INFO] [sprout::devtree] [CPU0] SPROUT: Init OK, returning context
[19845350094] [INFO] [sprout::devtree] [CPU0] SPROUT: build() called
[19846186059] [INFO] [sprout::devtree::x86_64] [CPU0] SPROUT: x86_64 platform enrichment... (v0.2)
[19849270227] [INFO] [sprout::devtree::x86_64] [CPU0] SPROUT: x86_64 enumerate done
[19850039126] [INFO] [sprout] [CPU0] SPROUT: About to create Supervisor...
[19850858198] [INFO] [sprout] [CPU0] SPROUT: Supervisor created, calling run_forever...
[19851779881] [INFO] [sprout::supervisor] [CPU0] SPROUT: Supervisor starting (minimal mode)...
[19852568867] [INFO] [sprout::supervisor] [CPU0] SPROUT: Discovering modules...
[19855538394] [INFO] [sprout::supervisor] [CPU0] SPROUT: Found 35 modules
[19904460422] [INFO] [sprout::supervisor] [CPU0] SPROUT: Module[0] = '/boot/sprout'
[19910246568] [INFO] [sprout::supervisor] [CPU0] SPROUT: Module[1] = '/boot/bristle'
[19914577777] [INFO] [sprout::supervisor] [CPU0] SPROUT: Module[2] = '/boot/rtc_cmos'
[19919006550] [INFO] [sprout::supervisor] [CPU0] SPROUT: Module[3] = '/boot/clock'
[19921824027] [INFO] [sprout::supervisor] [CPU0] SPROUT: Discovered app: /boot/clock
[19925648767] [INFO] [sprout::supervisor] [CPU0] SPROUT: Module[4] = '/boot/taskman'
[19930268888] [INFO] [sprout::supervisor] [CPU0] SPROUT: Module[5] = '/boot/ps2_kbd'
[19934475555] [INFO] [sprout::supervisor] [CPU0] SPROUT: Module[6] = '/boot/echo'
[19938770406] [INFO] [sprout::supervisor] [CPU0] SPROUT: Module[7] = '/boot/bloom'
[19943204200] [INFO] [sprout::supervisor] [CPU0] SPROUT: Module[8] = '/boot/ps2_mouse'
[19947163888] [INFO] [sprout::supervisor] [CPU0] SPROUT: Module[9] = '/boot/display_bootfb'
[19951786061] [INFO] [sprout::supervisor] [CPU0] SPROUT: Module[10] = '/boot/display_virtio_gpu'
[19956871471] [INFO] [sprout::supervisor] [CPU0] SPROUT: Module[11] = '/boot/fontd'
[19961512588] [INFO] [sprout::supervisor] [CPU0] SPROUT: Module[12] = '/boot/blossom'
[19966160367] [INFO] [sprout::supervisor] [CPU0] SPROUT: Module[13] = '/boot/flytrap'
[19970733222] [INFO] [sprout::supervisor] [CPU0] SPROUT: Module[14] = '/boot/virtio_netd'
[19988809080] [INFO] [sprout::supervisor] [CPU0] SPROUT: Module[15] = '/boot/rtl8168d'
[19993151379] [INFO] [sprout::supervisor] [CPU0] SPROUT: Module[16] = '/boot/netd'
[19997499078] [INFO] [sprout::supervisor] [CPU0] SPROUT: Module[17] = '/boot/fetchd'
[20001757205] [INFO] [sprout::supervisor] [CPU0] SPROUT: Module[18] = '/boot/anther'
[20006105697] [INFO] [sprout::supervisor] [CPU0] SPROUT: Module[19] = '/boot/photosynthesis'
[20010767278] [INFO] [sprout::supervisor] [CPU0] SPROUT: Module[20] = '/boot/ahci_disk'
[20014998255] [INFO] [sprout::supervisor] [CPU0] SPROUT: Module[21] = '/boot/iso9660d'
[20019573548] [INFO] [sprout::supervisor] [CPU0] SPROUT: Module[22] = '/boot/virtio_sound'
[20023640098] [INFO] [sprout::supervisor] [CPU0] SPROUT: Module[23] = '/boot/hdaudio'
[20027873936] [INFO] [sprout::supervisor] [CPU0] SPROUT: Module[24] = '/boot/pci_stubd'
[20032186167] [INFO] [sprout::supervisor] [CPU0] SPROUT: Module[25] = '/boot/beeper'
[20038743229] [INFO] [sprout::supervisor] [CPU0] SPROUT: Module[26] = '/boot/nectar'
[20043058040] [INFO] [sprout::supervisor] [CPU0] SPROUT: Module[27] = '/assets/wallpapers/clouds.bmp'
[20047360471] [INFO] [sprout::supervisor] [CPU0] SPROUT: Module[28] = '/assets/wallpapers/flower.bmp'
[20051583575] [INFO] [sprout::supervisor] [CPU0] SPROUT: Module[29] = '/assets/wallpapers/leather.bmp'
[20056161899] [INFO] [sprout::supervisor] [CPU0] SPROUT: Module[30] = '/assets/wallpapers/linen.bmp'
[20060830538] [INFO] [sprout::supervisor] [CPU0] SPROUT: Module[31] = '/assets/fonts/NotoSans-Regular.ttf'
[20065665744] [INFO] [sprout::supervisor] [CPU0] SPROUT: Module[32] = '/assets/themes/genie_circles.wasm'
[20069890169] [INFO] [sprout::supervisor] [CPU0] SPROUT: Module[33] = '/assets/cursors/future/default.svg'
[20074977161] [INFO] [sprout::supervisor] [CPU0] SPROUT: Module[34] = '/boot/locale.conf'
[20077816178] [INFO] [sprout::registry] [CPU0] SPROUT: Scanning boot modules...
[20098054516] [INFO] [sprout::registry] [CPU0] SPROUT: Registering driver 'dev.rtc.Cmos' -> '/boot/rtc_cmos' (fallback)
[20207375162] [INFO] [sprout::registry] [CPU0] SPROUT: Registry scan complete. Found 1 drivers.
[20214275636] [INFO] [kernel::task::loader] [CPU0] Loading module: /boot/pci_stubd
[20214874612] [DEBUG] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20215781804] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[20221051651] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=205000 exec=false
[20222274594] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=206000 exec=false
[20231120182] [DEBUG] [kernel::task::scheduler::spawn] [CPU0] SCHED: Task 5 (user task/process) assigned to CPU 0
[20233816090] [INFO] [sprout::pipelines] [CPU0] SPROUT: Spawned pci_stubd (PID=5)
[20242840473] [DEBUG] [kernel::task::scheduler::spawn] [CPU0] Trampoline entered. Arg: 0xffffffffb0024a20
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[20243776489] [INFO] [bran::arch::x86_64::enter_user] [CPU0] Entering user mode tid=5 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562074941 RSP_BEFORE=18446744072368293424 RFLAGS_BEFORE=134 CR3_BEFORE=50790400 fs_base=0 gs_base=18446744071564225888
[20247105713] [INFO] [pci_stubd] [CPU0] pci_stubd: starting pci-id matcher
[20252826451] [INFO] [kernel::task::loader] [CPU0] Loading module: /boot/rtc_cmos
[20253347207] [DEBUG] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[20254266344] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[20259613977] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=205000 exec=false
[20261337875] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=207000 exec=false
[20268420384] [DEBUG] [kernel::task::scheduler::spawn] [CPU0] SCHED: Task 6 (user task/process) assigned to CPU 0
[20269797105] [INFO] [sprout::pipelines] [CPU0] SPROUT: Spawned rtc_cmos (PID=6)
[20271358732] [INFO] [sprout::pipelines] [CPU0] SPROUT: Setting up storage pipeline...
[20278355204] [DEBUG] [kernel::task::scheduler::spawn] [CPU0] Trampoline entered. Arg: 0xffffffffb0024a20
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xc2
[20279398379] [INFO] [bran::arch::x86_64::enter_user] [CPU0] Entering user mode tid=6 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562074941 RSP_BEFORE=18446744072368358960 RFLAGS_BEFORE=134 CR3_BEFORE=50905088 fs_base=0 gs_base=18446744071564225888
[20283731934] [INFO] [rtc_cmos] [CPU0] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ffeb0 rip=0x201027 rflags=0x202
[20284688099] [INFO] [rtc_cmos] [CPU0] Starting... arg=c2
[20286640558] [INFO] [rtc_cmos] [CPU0] Serving device ID: ThingId([194, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[20290610141] [INFO] [rtc_cmos] [CPU0] RTC: 2026-02-11 17:01:55 = 1770829315 unix_secs
[20291923503] [INFO] [kernel::time] [CPU0] System clock anchored: unix_secs=1770829315, mono_ns=10145668896, offset=1770829304854331104ns
[20293274392] [INFO] [rtc_cmos] [CPU0] System clock anchored
[20307603165] [INFO] [rtc_cmos] [CPU0] RTC: Set sys.TimeState = 1 (Anchored)
[20340120177] [INFO] [rtc_cmos] [CPU0] Publishing time. Entering maintenance loop.
[20363623707] [INFO] [kernel::task::loader] [CPU0] Loading module: /boot/ahci_disk
[20364207753] [DEBUG] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[20365102864] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[20372319922] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=207000 exec=false
[20375020605] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=20a000 exec=false
[20383549850] [DEBUG] [kernel::task::scheduler::spawn] [CPU0] SCHED: Task 7 (user task/process) assigned to CPU 0
[20398895440] [INFO] [sprout::pipelines] [CPU0] SPROUT: Spawned ahci_disk (PID=7)
[20403267274] [DEBUG] [kernel::task::scheduler::spawn] [CPU0] Trampoline entered. Arg: 0xffffffffb0024a20
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[20404174727] [INFO] [bran::arch::x86_64::enter_user] [CPU0] Entering user mode tid=7 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562074941 RSP_BEFORE=18446744072368424496 RFLAGS_BEFORE=134 CR3_BEFORE=51023872 fs_base=0 gs_base=18446744071564225888
[20407199812] [INFO] [ahci_disk] [CPU0] AHCI: Starting AHCI/SATA disk driver v1
[20411053187] [INFO] [sprout::pipelines] [CPU0] SPROUT: Failed to spawn ata_disk: ENOENT
[20412469748] [INFO] [kernel::task::loader] [CPU0] Loading module: /boot/iso9660d
[20412919480] [DEBUG] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[20413782158] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[20422614841] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=209000 exec=false
[20424652207] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=20b000 exec=false
[20432434880] [DEBUG] [kernel::task::scheduler::spawn] [CPU0] SCHED: Task 8 (user task/process) assigned to CPU 0
[20434257306] [INFO] [sprout::pipelines] [CPU0] SPROUT: Spawned iso9660d (PID=8)
[20436197441] [INFO] [sprout::pipelines] [CPU0] SPROUT: Setting up audio pipeline...
[20441595743] [INFO] [ahci_disk] [CPU0] AHCI: Found 6 PCI functions
[20443355066] [DEBUG] [kernel::task::scheduler::spawn] [CPU0] Trampoline entered. Arg: 0xffffffffb0024a20
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[20444431856] [INFO] [bran::arch::x86_64::enter_user] [CPU0] Entering user mode tid=8 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562074941 RSP_BEFORE=18446744072368490032 RFLAGS_BEFORE=134 CR3_BEFORE=51163136 fs_base=0 gs_base=18446744071564225888
[20447549560] [INFO] [iso9660d] [CPU0] ISO9660D: Starting ISO9660 mount service...
[20456293428] [INFO] [iso9660d] [CPU0] ISO9660D: Found 0 block devices
[20457185314] [INFO] [iso9660d] [CPU0] ISO9660D: No ISO9660 filesystems found
[20458176169] [INFO] [iso9660d] [CPU0] ISO9660D: Entering service loop with 0 mounts
[20459574431] [INFO] [sprout::pipelines] [CPU0] SPROUT: No Sound device found
[20460381383] [INFO] [sprout::pipelines] [CPU0] SPROUT: Setting up display pipeline...
[20487280075] [INFO] [sprout::pipelines] [CPU0] SPROUT: Using boot framebuffer (fallback)
[20488745409] [INFO] [sprout::pipelines] [CPU0] SPROUT: Display backend: BootFB (1920x1080 stride=7680)
[21143331916] [INFO] [ahci_disk] [CPU0] AHCI: Found AHCI controller at PCI func ThingId([198, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[21150543141] [INFO] [ahci_disk] [CPU0] AHCI: BAR5=0x81080000
[21153400069] [INFO] [kernel::syscall::handlers::device] [CPU0] DEVICE: task 7 claimed device 198 (handle 0)
[21154458295] [INFO] [ahci_disk] [CPU0] AHCI: Claimed PCI device ThingId([198, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) handle=0
[21158884246] [INFO] [kernel::syscall::handlers::device] [CPU0] DEVICE: Mapped BAR5 phys=0x81080000 size=0x1000 -> virt=0x10000000
[21160160866] [INFO] [ahci_disk] [CPU0] AHCI: Mapped ABAR at 0x10000000
[21161788418] [INFO] [ahci_disk] [CPU0] AHCI: Version 1.0, 6 ports, 32 slots, 64-bit: true
[21162831200] [INFO] [ahci_disk] [CPU0] AHCI: Ports implemented: 0x3f
[21163628643] [INFO] [ahci_disk] [CPU0] AHCI: DMA virt=0x20b000
[21166033436] [INFO] [ahci_disk] [CPU0] AHCI: DMA phys=0x30b8000
[21167398119] [INFO] [ahci_disk] [CPU0] AHCI: Probing port 0...
[21168288662] [INFO] [ahci_disk] [CPU0] AHCI: Port 0 - no device
[21169024565] [INFO] [ahci_disk] [CPU0] AHCI: Probing port 1...
[21169626978] [INFO] [ahci_disk] [CPU0] AHCI: Port 1 - no device
[21170226352] [INFO] [ahci_disk] [CPU0] AHCI: Probing port 2...
[21171014032] [INFO] [ahci_disk] [CPU0] AHCI: Port 2 - SATAPI drive (sig=0xeb140101)
[21205195178] [INFO] [kernel::task::loader] [CPU0] Loading module: /boot/display_bootfb
[21205809846] [DEBUG] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[21206691036] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[21212004366] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=205000 exec=false
[21214025671] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=207000 exec=false
[21222576215] [DEBUG] [kernel::task::scheduler::spawn] [CPU0] SCHED: Task 9 (user task/process) assigned to CPU 0
[21224459707] [INFO] [sprout::pipelines] [CPU0] SPROUT: Spawned display driver '/display_bootfb' (PID=9)
[21232122960] [DEBUG] [kernel::task::scheduler::spawn] [CPU0] Trampoline entered. Arg: 0xffffffffb0024a20
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x50004
[21233227270] [INFO] [bran::arch::x86_64::enter_user] [CPU0] Entering user mode tid=9 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562074941 RSP_BEFORE=18446744072368555568 RFLAGS_BEFORE=134 CR3_BEFORE=59596800 fs_base=0 gs_base=18446744071564225888
[21237545873] [INFO] [display_bootfb] [CPU0] display_bootfb: starting (drv_req_r=4, drv_resp_w=5)
[21245583761] [INFO] [ahci_disk] [CPU0] AHCI: Registered ATAPI block device 209 port=2 model='                                        ' rpc_port=1
[21276484361] [INFO] [ahci_disk] [CPU0] AHCI: Probing port 3...
[21277393924] [INFO] [ahci_disk] [CPU0] AHCI: Port 3 - no device
[21277979845] [INFO] [ahci_disk] [CPU0] AHCI: Probing port 4...
[21278551805] [INFO] [ahci_disk] [CPU0] AHCI: Port 4 - no device
[21279135664] [INFO] [ahci_disk] [CPU0] AHCI: Probing port 5...
[21279784861] [INFO] [ahci_disk] [CPU0] AHCI: Port 5 - no device
[21280450908] [INFO] [ahci_disk] [CPU0] AHCI: Found 1 SATA disk(s)
[21281094359] [INFO] [ahci_disk] [CPU0] AHCI: Entering RPC service loop
[21303753980] [INFO] [sprout::pipelines] [CPU0] SPROUT: Setting up input pipeline (keyboard + mouse)...
[21306124156] [INFO] [sprout::pipelines] [CPU0] SPROUT: Created kbd_raw port (w=7, r=8)
[21307832916] [INFO] [sprout::pipelines] [CPU0] SPROUT: Created mouse_raw port (w=9, r=10)
[21309148251] [INFO] [kernel::task::loader] [CPU0] Loading module: /boot/ps2_kbd
[21309593759] [DEBUG] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21310422801] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[21314902933] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=204000 exec=false
[21316140614] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=205000 exec=false
[21323657883] [DEBUG] [kernel::task::scheduler::spawn] [CPU0] SCHED: Task 10 (user task/process) assigned to CPU 0
[21325131026] [INFO] [sprout::pipelines] [CPU0] SPROUT: Spawned ps2_kbd (PID=10)
[21326751871] [INFO] [kernel::task::loader] [CPU0] Loading module: /boot/ps2_mouse
[21327233224] [DEBUG] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21328280859] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[21333144397] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=204000 exec=false
[21334436619] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=205000 exec=false
[21342479242] [DEBUG] [kernel::task::scheduler::spawn] [CPU0] SCHED: Task 11 (user task/process) assigned to CPU 0
[21344716254] [INFO] [sprout::pipelines] [CPU0] SPROUT: Spawned ps2_mouse (PID=11)
[21347744516] [INFO] [kernel::task::loader] [CPU0] Loading module: /boot/bristle
[21348247773] [DEBUG] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21349244684] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[21355246763] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=205000 exec=false
[21357095955] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=207000 exec=false
[21365258555] [DEBUG] [kernel::task::scheduler::spawn] [CPU0] SCHED: Task 12 (user task/process) assigned to CPU 0
[21367217737] [INFO] [sprout::pipelines] [CPU0] SPROUT: Spawned bristle (PID=12)
[21372301226] [INFO] [kernel::syscall::handlers::device] [CPU0] DEVICE: task 9 claimed device 175 (handle 1)
[21389204777] [INFO] [kernel::syscall::handlers::device] [CPU0] DEVICE: Mapped BAR0 phys=0x80000000 size=0x7e9000 -> virt=0x10001000
[21952565599] [DEBUG] [kernel::task::scheduler::spawn] [CPU0] Trampoline entered. Arg: 0xffffffffb0024a20
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x7
[21953785546] [INFO] [bran::arch::x86_64::enter_user] [CPU0] Entering user mode tid=10 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562074941 RSP_BEFORE=18446744072368621104 RFLAGS_BEFORE=134 CR3_BEFORE=59981824 fs_base=0 gs_base=18446744071564225888
[21958887147] [INFO] [ps2_kbd] [CPU0] ps2_kbd: online (handle=7)
[21961934475] [DEBUG] [kernel::task::scheduler::spawn] [CPU0] Trampoline entered. Arg: 0xffffffffb004d570
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x9
[21963079214] [INFO] [bran::arch::x86_64::enter_user] [CPU0] Entering user mode tid=11 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562074941 RSP_BEFORE=18446744072368686640 RFLAGS_BEFORE=134 CR3_BEFORE=60092416 fs_base=0 gs_base=18446744071564225888
[21967466448] [INFO] [ps2_mouse] [CPU0] ps2_mouse: online (handle=9)
[21968427507] [INFO] [ps2_mouse] [CPU0] ps2_mouse: enabling aux port
[21969573159] [DEBUG] [kernel::task::scheduler::spawn] [CPU0] Trampoline entered. Arg: 0xffffffffb004c910
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x8000a000b000d
[21970639691] [INFO] [bran::arch::x86_64::enter_user] [CPU0] Entering user mode tid=12 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562074941 RSP_BEFORE=18446744072368752176 RFLAGS_BEFORE=134 CR3_BEFORE=60203008 fs_base=0 gs_base=18446744071564225888
[21975462996] [INFO] [bristle] [CPU0] bristle: online (kbd=8, mouse=10, evt=11, echo=13)
[21978461170] [INFO] [bristle] [CPU0] bristle: created broadcast topic 0
[21989563635] [INFO] [kernel::root::service] [CPU0] ROOT STATS: iter=3000 nodes=218 watches=0 history=1024 journal=927 symbols=221 drops=0
[21992550457] [INFO] [ps2_kbd] [CPU0] ps2_kbd: created driver node 216
[21994982724] [INFO] [kernel::syscall::handlers::device] [CPU0] DEVICE: task subscribed to vector 0x21
[21996661667] [INFO] [ps2_kbd] [CPU0] ps2_kbd: subscribed to IRQ1 (vector 0x21)
[21999323374] [INFO] [bristle] [CPU0] bristle: registered in graph as svc.Input (id=217)
[22005087195] [INFO] [sprout::pipelines] [CPU0] SPROUT: Writing bloom BS: drv_req=3, drv_resp=6, bristle_evt=12
[22029169592] [INFO] [display_bootfb] [CPU0] display_bootfb: created back buffer (size=8294400)
[22040982779] [INFO] [sprout::pipelines] [CPU0] SPROUT: Bloom handles via BS=212 backend=BootFB
[22042373182] [INFO] [kernel::task::loader] [CPU0] Loading module: /boot/bloom
[22042924105] [DEBUG] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[22044067421] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[22246053750] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=2d2000 exec=false
[22263820007] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=2e6000 exec=false
[22273282634] [DEBUG] [kernel::task::scheduler::spawn] [CPU0] SCHED: Task 13 (user task/process) assigned to CPU 0
[22277484925] [INFO] [sprout::pipelines] [CPU0] SPROUT: Spawned bloom (PID=13)
[22279807599] [INFO] [kernel::task::loader] [CPU0] Loading module: /boot/echo
[22280246278] [DEBUG] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[22281070454] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[22285880298] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=204000 exec=false
[22287588009] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=206000 exec=false
[22295805750] [DEBUG] [kernel::task::scheduler::spawn] [CPU0] SCHED: Task 14 (user task/process) assigned to CPU 0
[22298349293] [INFO] [sprout::pipelines] [CPU0] SPROUT: Spawned echo (PID=14)
[22299565137] [INFO] [sprout::pipelines] [CPU0] SPROUT: Input pipeline ready (keyboard + mouse)
[22301187418] [INFO] [kernel::task::loader] [CPU0] Loading module: /boot/blossom
[22301638852] [DEBUG] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[22302491236] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[22324642467] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=219000 exec=false
[22327566476] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=21c000 exec=false
[22336179633] [DEBUG] [kernel::task::scheduler::spawn] [CPU0] SCHED: Task 15 (user task/process) assigned to CPU 0
[22338918227] [INFO] [sprout::pipelines] [CPU0] SPROUT: Spawned blossom (PID=15)
[22340681965] [INFO] [kernel::task::loader] [CPU0] Loading module: /boot/fontd
[22341144001] [DEBUG] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[22342007031] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[22373831699] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=226000 exec=false
[22379877460] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=22d000 exec=false
[22388632472] [DEBUG] [kernel::task::scheduler::spawn] [CPU0] SCHED: Task 16 (user task/process) assigned to CPU 0
[22412922104] [INFO] [ps2_kbd] [CPU0] ps2_kbd: entering interrupt-driven loop
[22417768479] [INFO] [ps2_mouse] [CPU0] ps2_mouse: controller cfg already correct (0x47)
[22418622470] [INFO] [ps2_mouse] [CPU0] ps2_mouse: sending enable command (0xF4)
[22420590578] [INFO] [bran::arch::x86_64::idt] [CPU0] IRQ12 fired (count=1)
[22425730598] [INFO] [ps2_mouse] [CPU0] ps2_mouse: enable ACK received (0xFA)
[22426706583] [DEBUG] [kernel::task::scheduler::spawn] [CPU0] Trampoline entered. Arg: 0xffffffffb0024a20
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xd4
[22427710463] [INFO] [bran::arch::x86_64::enter_user] [CPU0] Entering user mode tid=13 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562074941 RSP_BEFORE=18446744072368818624 RFLAGS_BEFORE=134 CR3_BEFORE=68657152 fs_base=0 gs_base=18446744071564225888
[22431535513] [INFO] [bloom::logging] [CPU0] bloom: logging initialized
[22462416953] [INFO] [bloom] [CPU0] bloom: [bloom] SIMD backend: SSE2 (x86_64)
[22464537530] [DEBUG] [kernel::task::scheduler::spawn] [CPU0] Trampoline entered. Arg: 0xffffffffb004c910
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xe
[22465517693] [INFO] [bran::arch::x86_64::enter_user] [CPU0] Entering user mode tid=14 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562074941 RSP_BEFORE=18446744072368884608 RFLAGS_BEFORE=130 CR3_BEFORE=69689344 fs_base=0 gs_base=18446744071564225888
[22468575245] [INFO] [echo] [CPU0] echo: starting up
[22470845620] [DEBUG] [kernel::task::scheduler::spawn] [CPU0] Trampoline entered. Arg: 0xffffffffb004c938
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[22471805867] [INFO] [bran::arch::x86_64::enter_user] [CPU0] Entering user mode tid=15 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562074941 RSP_BEFORE=18446744072368951280 RFLAGS_BEFORE=134 CR3_BEFORE=69804032 fs_base=0 gs_base=18446744071564225888
[22475250882] [INFO] [blossom] [CPU0] BLOSSOM: Starting SVG Cache Service
[22476592204] [INFO] [blossom] [CPU0] BLOSSOM: Init UI pipeline...
[22478871379] [DEBUG] [kernel::task::scheduler::spawn] [CPU0] Trampoline entered. Arg: 0xffffffffb004d878
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[22479885748] [INFO] [bran::arch::x86_64::enter_user] [CPU0] Entering user mode tid=16 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562074941 RSP_BEFORE=18446744072369016944 RFLAGS_BEFORE=130 CR3_BEFORE=70008832 fs_base=0 gs_base=18446744071564225888
[22483247076] [INFO] [fontd] [CPU0] FONTD: Starting Graph-Native Font Service v2 (Atlas-based IPC)
[22490157143] [INFO] [sprout::pipelines] [CPU0] SPROUT: Spawned fontd (PID=16)
[22492054567] [INFO] [sprout::pipelines] [CPU0] SPROUT: Setting up network pipeline...
[22504875293] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: ptr=0x7fbfb0
[22506206355] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: validating range len=48
[22507423907] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: copyin success. mode=1 start_seq=0
[22508839604] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=224 subj_lo=0
[22532478322] [INFO] [bloom] [CPU0] [bloom] EARLY boot args: bristle_evt=12 arg_req=3 arg_resp=6
[22539421854] [INFO] [echo] [CPU0] echo: dynamically subscribed to input topic 0 via port 20
[22540656602] [INFO] [echo] [CPU0] echo: ready for Bristle events (main loop using handle 20)
[22581943806] [INFO] [sprout::pipelines] [CPU0] SPROUT: Found NIC device ThingId([190, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[22583727941] [INFO] [kernel::task::loader] [CPU0] Loading module: /boot/virtio_netd
[22584207937] [DEBUG] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[22585109326] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[22593976045] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=208000 exec=false
[22596807201] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=20b000 exec=false
[22605847633] [DEBUG] [kernel::task::scheduler::spawn] [CPU0] SCHED: Task 17 (user task/process) assigned to CPU 0
[22607989074] [INFO] [sprout::pipelines] [CPU0] SPROUT: Spawned virtio_netd (PID=17)
[22609421313] [INFO] [sprout::pipelines] [CPU0] SPROUT: spawn_net_stack_services start
[22610750703] [INFO] [kernel::task::loader] [CPU0] Loading module: /boot/netd
[22611239839] [DEBUG] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[22612227792] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[22639586661] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=21d000 exec=false
[22644772917] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=223000 exec=false
[22672280507] [DEBUG] [kernel::task::scheduler::spawn] [CPU0] SCHED: Task 18 (user task/process) assigned to CPU 0
[22674330299] [INFO] [sprout::pipelines] [CPU0] SPROUT: Spawned netd (PID=18)
[22676397455] [INFO] [kernel::task::loader] [CPU0] Loading module: /boot/anther
[22676895228] [DEBUG] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[22677890231] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[22770005069] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=269000 exec=false
[22806097627] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=292000 exec=false
[22815259491] [DEBUG] [kernel::task::scheduler::spawn] [CPU0] SCHED: Task 19 (user task/process) assigned to CPU 0
[22817541425] [INFO] [sprout::pipelines] [CPU0] SPROUT: Spawned anther (PID=19)
[22819318289] [INFO] [kernel::task::loader] [CPU0] Loading module: /boot/nectar
[22819788635] [DEBUG] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[22820711456] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[22856767878] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=226000 exec=false
[22861043661] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=22a000 exec=false
[22870374518] [DEBUG] [kernel::task::scheduler::spawn] [CPU0] SCHED: Task 20 (user task/process) assigned to CPU 0
[22872812480] [INFO] [sprout::pipelines] [CPU0] SPROUT: Spawned nectar (PID=20)
[22874680967] [INFO] [kernel::task::loader] [CPU0] Loading module: /boot/fetchd
[22875145246] [DEBUG] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[22876083849] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[22884119243] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=208000 exec=false
[22887022275] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=20b000 exec=false
[22895255347] [DEBUG] [kernel::task::scheduler::spawn] [CPU0] SCHED: Task 21 (user task/process) assigned to CPU 0
[22897443615] [INFO] [sprout::pipelines] [CPU0] SPROUT: Spawned fetchd (PID=21)
[22899378508] [INFO] [kernel::task::loader] [CPU0] Loading module: /boot/clock
[22899907123] [DEBUG] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[22900768191] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[22910874281] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=20a000 exec=false
[22915682620] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=20f000 exec=false
[22924577119] [DEBUG] [kernel::task::scheduler::spawn] [CPU0] SCHED: Task 22 (user task/process) assigned to CPU 0
[22926603731] [INFO] [sprout::pipelines] [CPU0] SPROUT: Spawned clock (PID=22)
[22928395256] [INFO] [kernel::task::loader] [CPU0] Loading module: /boot/taskman
[22928880886] [DEBUG] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[22929704080] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[22942098782] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=20c000 exec=false
[22946108676] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=211000 exec=false
[22954845917] [DEBUG] [kernel::task::scheduler::spawn] [CPU0] SCHED: Task 23 (user task/process) assigned to CPU 0
[22956779198] [INFO] [sprout::pipelines] [CPU0] SPROUT: Spawned taskman (PID=23)
[22958581652] [INFO] [kernel::task::loader] [CPU0] Loading module: /boot/flytrap
[22959248974] [DEBUG] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[22960467398] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[23010637070] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=237000 exec=false
[23023274037] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=245000 exec=false
[23032648688] [DEBUG] [kernel::task::scheduler::spawn] [CPU0] SCHED: Task 24 (user task/process) assigned to CPU 0
[23034860680] [INFO] [sprout::pipelines] [CPU0] SPROUT: Spawned flytrap (PID=24)
[23035922137] [INFO] [sprout::supervisor] [CPU0] SPROUT: Startup complete. Entering monitor loop.
[23051870528] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: ptr=0x7fbfb0
[23052704600] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: validating range len=48
[23053325600] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: copyin success. mode=1 start_seq=0
[23054234071] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=231 subj_lo=0
[23060350374] [INFO] [fontd] [CPU0] FONTD: Service node created, req=15, resp=18
[23062455068] [DEBUG] [kernel::task::scheduler::spawn] [CPU0] Trampoline entered. Arg: 0xffffffffb004c0f8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xbe
[23063406776] [INFO] [bran::arch::x86_64::enter_user] [CPU0] Entering user mode tid=17 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562074941 RSP_BEFORE=18446744072369110400 RFLAGS_BEFORE=130 CR3_BEFORE=70819840 fs_base=0 gs_base=18446744071564225888
[23067199869] [INFO] [virtio_netd] [CPU0] VIRTIO_NETD: Starting VirtIO-NET driver service...
[23068173304] [INFO] [virtio_netd::driver] [CPU0] VirtIO-NET: Searching for NIC device...
[23070271480] [DEBUG] [kernel::task::scheduler::spawn] [CPU0] Trampoline entered. Arg: 0xffffffffb004c938
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[23071421918] [INFO] [bran::arch::x86_64::enter_user] [CPU0] Entering user mode tid=18 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562074941 RSP_BEFORE=18446744072369175936 RFLAGS_BEFORE=130 CR3_BEFORE=70955008 fs_base=0 gs_base=18446744071564225888
[23075271101] [INFO] [netd] [CPU0] NETD: Starting network stack service...
[23076216969] [INFO] [netd] [CPU0] NETD: Looking for virtio_netd driver service...
[23078235982] [DEBUG] [kernel::task::scheduler::spawn] [CPU0] Trampoline entered. Arg: 0xffffffffb004d878
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[23079215161] [INFO] [bran::arch::x86_64::enter_user] [CPU0] Entering user mode tid=19 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562074941 RSP_BEFORE=18446744072369241760 RFLAGS_BEFORE=134 CR3_BEFORE=71352320 fs_base=0 gs_base=18446744071564225888
[23083029542] [INFO] [anther] [CPU0] anther: Starting HTTP server (ThingOS anther v0.1)
[23085245995] [INFO] [anther] [CPU0] anther: Starting server mode on port 80...
[23087670339] [DEBUG] [kernel::task::scheduler::spawn] [CPU0] Trampoline entered. Arg: 0xffffffffb00759f8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[23088659537] [INFO] [bran::arch::x86_64::enter_user] [CPU0] Entering user mode tid=20 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562074941 RSP_BEFORE=18446744072369307536 RFLAGS_BEFORE=134 CR3_BEFORE=72040448 fs_base=0 gs_base=18446744071564225888
[23091764592] [INFO] [nectar] [CPU0] NECTAR: Started.
[23094022859] [DEBUG] [kernel::task::scheduler::spawn] [CPU0] Trampoline entered. Arg: 0xffffffffb0075a48
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[23094993561] [INFO] [bran::arch::x86_64::enter_user] [CPU0] Entering user mode tid=21 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562074941 RSP_BEFORE=18446744072369376896 RFLAGS_BEFORE=130 CR3_BEFORE=72302592 fs_base=0 gs_base=18446744071564225888
[23098853532] [INFO] [fetchd] [CPU0] FETCHD: Starting IP address display...
[23099723201] [INFO] [fetchd] [CPU0] FETCHD: Waiting for UI Root (Compositor)...
[23101755758] [DEBUG] [kernel::task::scheduler::spawn] [CPU0] Trampoline entered. Arg: 0xffffffffb00766e8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[23102808066] [INFO] [bran::arch::x86_64::enter_user] [CPU0] Entering user mode tid=22 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562074941 RSP_BEFORE=18446744072369442432 RFLAGS_BEFORE=130 CR3_BEFORE=72437760 fs_base=0 gs_base=18446744071564225888
[23106692696] [DEBUG] [kernel::task::scheduler::spawn] [CPU0] Trampoline entered. Arg: 0xffffffffb00783f8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[23107875183] [INFO] [bran::arch::x86_64::enter_user] [CPU0] Entering user mode tid=24 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562074941 RSP_BEFORE=18446744072369573504 RFLAGS_BEFORE=130 CR3_BEFORE=72749056 fs_base=0 gs_base=18446744071564225888
[23110822199] [INFO] [flytrap] [CPU0] FLYTRAP: Starting unified content provider service...
[23111816518] [INFO] [flytrap] [CPU0] FLYTRAP: Service contract validated - graph-native asset watcher
[23112821805] [INFO] [flytrap] [CPU0] FLYTRAP: Initializing Limine module content source...
[23124950832] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: ptr=0x7f9fb0
[23125584674] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: validating range len=48
[23126440264] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: copyin success. mode=0 start_seq=0
[23127099989] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=233 pred=0 subj_lo=0
[23130856453] [INFO] [virtio_netd::driver] [CPU0] VirtIO-NET: Found NIC device tbe
[23131738330] [INFO] [virtio::device] [CPU0] VirtIO: device::new(0xbe) - claiming...
[23132403857] [INFO] [kernel::syscall::handlers::device] [CPU0] DEVICE: task 17 claimed device 190 (handle 2)
[23133481590] [INFO] [virtio::device] [CPU0] VirtIO: claimed, handle=2
[23136616571] [INFO] [anther::net_client] [CPU0] anther: Network stack not found
[23137571018] [INFO] [anther] [CPU0] anther: Waiting for network stack...
[23154049765] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: ptr=0x7fbfb0
[23154671577] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: validating range len=48
[23155282878] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: copyin success. mode=1 start_seq=0
[23156026371] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=240 subj_lo=0
[23173252308] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: ptr=0x7f9fb0
[23174023774] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: validating range len=48
[23174754974] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: copyin success. mode=0 start_seq=0
[23175455330] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=241 pred=0 subj_lo=0
[23187195927] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: ptr=0x7fbfb0
[23187854735] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: validating range len=48
[23188933969] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: copyin success. mode=1 start_seq=0
[23189713600] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=244 subj_lo=0
[23205551264] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: ptr=0x7f9fb0
[23206211918] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: validating range len=48
[23207006502] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: copyin success. mode=0 start_seq=0
[23207657796] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=246 pred=0 subj_lo=0
[23250958304] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: ptr=0x7fbfb0
[23251593043] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: validating range len=48
[23252200194] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: copyin success. mode=1 start_seq=0
[23252966420] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=248 subj_lo=0
[23256012723] [INFO] [fontd] [CPU0] FONTD: Opened ASSET watch (handle=232) for kind 'Asset'
[23256975631] [INFO] [fontd] [CPU0] FONTD: Service ready
[23263147249] [INFO] [virtio::device] [CPU0] VirtIO: common_bar=4 common_off=0x0 notify_bar=4 notify_off=0x3000 mult=4
[23276486751] [INFO] [bloom::compositor] [CPU0] bloom: compositor bytespace 204 (1920x1080 stride=7680 format=2)
[23289081011] [INFO] [kernel::task::scheduler] [CPU0] PROF: sched 2s: graph_flush calls=7 items=57 avg_us=125476 max_us=651999 slow=5 trylock_miss=0 qlen=59 q_hwm=59 q_drop_state=0 q_evict=0
[23293453194] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: ptr=0x7fbfb0
[23294207641] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: validating range len=48
[23294868343] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: copyin success. mode=1 start_seq=0
[23295583223] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=251 subj_lo=0
[23300839547] [INFO] [virtio::device] [CPU0] VirtIO: mapping common BAR4...
[23302061416] [INFO] [kernel::syscall::handlers::device] [CPU0] DEVICE: Mapped BAR4 phys=0xc000000000 size=0x4000 -> virt=0x10fd5000
[23303555809] [INFO] [virtio::device] [CPU0] VirtIO: common_cfg at 0x10fd5000
[23305060388] [INFO] [virtio::device] [CPU0] VirtIO: notify_cfg at 0x10fd8000
[23306881602] [INFO] [virtio::device] [CPU0] VirtIO: device_cfg = Some(285044736)
[23307803370] [INFO] [virtio::device] [CPU0] VirtIO: allocating DMA command buffer...
[23311142927] [INFO] [virtio::device] [CPU0] VirtIO: cmd_buf virt=0x10fd9000 phys=0x45fe000
[23311998880] [INFO] [virtio::device] [CPU0] VirtIO: device::new complete
[23314244542] [INFO] [virtio_netd::driver] [CPU0] VirtIO-NET: Device features 0x30bf8024
[23316037518] [INFO] [virtio_netd::driver] [CPU0] VirtIO-NET: MAC 52:54:00:12:34:56
[23317130492] [INFO] [virtio_netd::driver] [CPU0] VirtIO-NET: Link UP
[23327951931] [INFO] [virtio_netd::driver] [CPU0] VirtIO-NET: RX queue 0 setup (size=32)
[23337266416] [INFO] [virtio_netd::driver] [CPU0] VirtIO-NET: TX queue 1 setup (size=32)
[23345838236] [INFO] [virtio_netd::driver] [CPU0] VirtIO-NET: Allocated RX pool (16 pages, 32 buffers)
[23348105049] [INFO] [virtio_netd::driver] [CPU0] VirtIO-NET: Allocated TX buffer (1 page)
[23350571981] [INFO] [virtio_netd::driver] [CPU0] VirtIO-NET: RX queue filled with 32 buffers
[23351822537] [INFO] [virtio_netd::driver] [CPU0] VirtIO-NET: DRIVER_OK set, device is live
[23352776222] [INFO] [virtio_netd::driver] [CPU0] VirtIO-NET: Driver initialized successfully
[23353714533] [INFO] [virtio_netd] [CPU0] VIRTIO_NETD: Driver initialized successfully
[23354644986] [INFO] [virtio_netd] [CPU0] VIRTIO_NETD: MAC 52:54:00:12:34:56
[23355412889] [INFO] [virtio_netd] [CPU0] VIRTIO_NETD: Waiting for link...
[23356399140] [INFO] [virtio_netd] [CPU0] VIRTIO_NETD: Link is UP
[23362192429] [INFO] [virtio_netd] [CPU0] VIRTIO_NETD: Created port (write=21, read=22)
[23367957265] [INFO] [virtio_netd] [CPU0] VIRTIO_NETD: Created RX port (write=23, read=24)
[23386185866] [INFO] [virtio_netd] [CPU0] VIRTIO_NETD: Created service node ThingId([235, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[23393501815] [INFO] [bloom::compositor] [CPU0] bloom: display backend: BootFB
[23397479327] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: ptr=0x7fbfb0
[23398091728] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: validating range len=48
[23398713036] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: copyin success. mode=1 start_seq=0
[23399432465] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=255 pred=0 subj_lo=0
[23405175042] [INFO] [netd] [CPU0] NETD: Found driver service node ThingId([235, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[23410189837] [INFO] [flytrap] [CPU0] FLYTRAP: Created Limine ContentSource node
[23411253626] [INFO] [flytrap] [CPU0] FLYTRAP: Performing initial boot module scan...
[23415656892] [INFO] [bloom::compositor] [CPU0] bloom: mapped size=8294400 (source=bytespace_info)
[23448647060] [INFO] [bloom::present] [CPU0] bloom: driver REGISTER (kind=1 caps=0x3)
[23451277280] [INFO] [bloom::present] [CPU0] display: negotiated proto 1.0 caps=DIRTY_RECTS|FULLFRAME max_rects=32
[23479495309] [INFO] [virtio_netd] [CPU0] VIRTIO_NETD: Published service - TX port=21, RX port=24
[23520211951] [INFO] [ps2_mouse] [CPU0] ps2_mouse: init done
[23520808409] [INFO] [kernel::syscall::handlers::device] [CPU0] DEVICE: task subscribed to vector 0x2c
[23521826118] [INFO] [ps2_mouse] [CPU0] ps2_mouse: subscribed to IRQ12 (vector 0x2c)
[23522580627] [INFO] [ps2_mouse] [CPU0] ps2_mouse: entering interrupt-driven loop
[23565847196] [INFO] [blossom] [CPU0] BLOSSOM: UI pipeline ready
[23606168831] [INFO] [blossom] [CPU0] BLOSSOM: Service node created, req=25, resp=28
[23607265643] [INFO] [blossom] [CPU0] BLOSSOM: Service ready
[23639568676] [INFO] [bloom] [CPU0] [bloom] ACQUIRED initial driver buffer: 0x117f7000 (bs_id=ThingId([214, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]))
[23672061662] [INFO] [stem::ui] [CPU0] UiBuilder: created root 244
[23687827019] [DEBUG] [kernel::task::scheduler::spawn] [CPU0] SCHED: Task 25 (user thread) assigned to CPU 0
[23690243626] [INFO] [bloom] [CPU0] bloom: spawned asset watcher (tid=25)
[23691076277] [INFO] [bloom::frame_loop] [CPU0] bloom: running (fps_target=60)
[23697943925] [INFO] [netd] [CPU0] NETD: Found driver service node ThingId([235, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
T:E670 [23709956628] [DEBUG] [bloom::painter_resources] [CPU0] [bloom] asset_watcher_entry: spawning sub-loaders
[23720117116] [DEBUG] [kernel::task::scheduler::spawn] [CPU0] SCHED: Task 26 (user thread) assigned to CPU 0
[23729883462] [DEBUG] [kernel::task::scheduler::spawn] [CPU0] SCHED: Task 27 (user thread) assigned to CPU 0
[23738672354] [DEBUG] [kernel::task::scheduler::spawn] [CPU0] SCHED: Task 28 (user thread) assigned to CPU 0
[23782587984] [INFO] [flytrap] [CPU0] FLYTRAP: Published new asset '/boot/sprout' (raw, 107024 bytes, hash=3d53683da6d7028d)
T:EBB0 T:EA20 T:D3E0 [23799068635] [INFO] [bloom] [CPU0] [bloom] dynamically subscribed to input topic 0 via port 30
[23800152856] [INFO] [bloom] [CPU0] [bloom] bristle_evt_handle = 30 (legacy was 12)
[23854803867] [INFO] [netd] [CPU0] NETD: Driver TX port=21, RX port=24, link_up=true, mtu=1500
[23857398151] [INFO] [netd] [CPU0] NETD: Connected to driver - MAC 52:54:00:12:34:56
[23862436750] [INFO] [netd] [CPU0] NETD: Created socket API port (write=31, read=32)
[23871235575] [INFO] [bloom] [CPU0] bloom: No VirtioGpu - using CPU composition mode
[23887355008] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: ptr=0x7f5570
[23887970491] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: validating range len=48
[23888549496] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: copyin success. mode=1 start_seq=0
[23889220081] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=281 subj_lo=0
[23912383140] [INFO] [fetchd] [CPU0] FETCHD: Found UI Root: 244
[23922981910] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: ptr=0x7f5570
[23923794718] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: validating range len=48
[23924497685] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: copyin success. mode=1 start_seq=0
[23925195987] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=255 pred=0 subj_lo=0
[23968816872] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: ptr=0x7f5570
[23969646623] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: validating range len=48
[23970346005] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: copyin success. mode=1 start_seq=0
[23971092947] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=285 subj_lo=0
[23981783816] [INFO] [netd] [CPU0] NETD: Published initial stack node ThingId([247, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) to graph
[23989923001] [INFO] [bloom] [CPU0] [bloom] Starting UI loop immediately (not waiting for fonts)
[24019114000] [INFO] [flytrap] [CPU0] FLYTRAP: Created File node '/boot/sprout' (107024 bytes, hash=3d53683da6d7028d)
[24032056867] [INFO] [netd] [CPU0] NETD: Starting DHCP...
[24033169780] [INFO] [netd::dhcp] [CPU0] DHCP: Starting discovery...
[24038395819] [INFO] [netd::ipc_device] [CPU0] IpcNicDevice: TX 304 bytes - IPv4 other
[24071101261] [INFO] [netd::dhcp] [CPU0] DHCP: Deconfigured
[24072463125] [DEBUG] [bloom::painter_resources] [CPU0] [bloom] wallpaper loader: loading flower.bmp
[24081331003] [DEBUG] [kernel::task::scheduler::spawn] [CPU0] SCHED: Task 29 (user thread) assigned to CPU 0
[24082964670] [DEBUG] [bloom::asset] [CPU0] [asset_bank] worker spawned tid=29 (priority=2)
[24084817521] [DEBUG] [bloom::painter_resources] [CPU0] [bloom] cursor loader: loading default cursor
[24087467931] [DEBUG] [bloom::painter_resources] [CPU0] [bloom] icon loader started
[24101001912] [INFO] [virtio_netd::driver] [CPU0] VirtIO-NET: TX 304 bytes
T:F600 [24107262669] [DEBUG] [bloom::asset] [CPU0] [asset_bank] worker started (priority bump)
[24109295564] [DEBUG] [bloom::asset] [CPU0] [asset_bank] load_wallpaper_immediate: flower.bmp
[24125850220] [INFO] [virtio_netd::driver] [CPU0] VirtIO-NET: RX frame! desc=0 len=600
[24160945549] [INFO] [virtio_netd] [CPU0] VIRTIO_NETD: Forwarding 590 byte frame (594 encoded) to netd rx_port=23
[24162857729] [INFO] [virtio_netd] [CPU0] VIRTIO_NETD: Frame forwarded successfully (594 bytes sent)
[24183839376] [INFO] [netd::ipc_device] [CPU0] IpcNicDevice: RX frame from driver, 590 bytes
[24190510518] [INFO] [netd::ipc_device] [CPU0] IpcNicDevice: TX 316 bytes - IPv4 other
[24194200874] [INFO] [virtio_netd::driver] [CPU0] VirtIO-NET: TX 316 bytes
[24235422281] [INFO] [virtio_netd::driver] [CPU0] VirtIO-NET: RX frame! desc=1 len=600
[24236426867] [INFO] [virtio_netd] [CPU0] VIRTIO_NETD: Forwarding 590 byte frame (594 encoded) to netd rx_port=23
[24238085100] [INFO] [virtio_netd] [CPU0] VIRTIO_NETD: Frame forwarded successfully (594 bytes sent)
[24251773446] [INFO] [netd::ipc_device] [CPU0] IpcNicDevice: RX frame from driver, 590 bytes
[24255286153] [INFO] [netd::dhcp] [CPU0] DHCP: Configuration received
[24256863136] [INFO] [netd] [CPU0] NETD: DHCP complete - IP: 10.0.2.15, Gateway: 10.0.2.2, DNS: 10.0.2.3
[24258051407] [INFO] [netd] [CPU0] NETD: DHCP configured (IP: 10.0.2.15), deferring graph updates to main loop
[24260777708] [INFO] [netd] [CPU0] NETD: Network stack ready, entering service loop
[24353705366] [INFO] [anther::net_client] [CPU0] anther: Connected to netd socket API (netd_port=31, our_write=33, our_read=34)
[24354884253] [INFO] [anther] [CPU0] anther: Connected to network stack
[24457986270] [INFO] [netd::socket_api] [CPU0] SOCKET_API: TCP_LISTEN on port 80
[24460493513] [INFO] [netd::socket_api] [CPU0] SOCKET_API: Listening on port 80, handle=1
[24465737007] [INFO] [anther] [CPU0] anther: Listening on port 80 (handle=1)

```
</details>
