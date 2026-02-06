# ✅ Scenario: Boot and System Verification

> Last run: 2026-02-06 20:55:48

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booting | ✅ | 366ms | <a href="./01/after.png"><img src="./01/after.png" width="150" /></a> - [💾](./01/registers.txt) |
| 2 | When I wait for the system to reach ready state | ✅ | 7654ms | <a href="./02/after.png"><img src="./02/after.png" width="150" /></a> [📜](./02/serial.log) [💾](./02/registers.txt) |
| 3 | Then the serial output should contain "SPROUT:" | ✅ | 1533ms | <a href="./03/after.png"><img src="./03/after.png" width="150" /></a> [📜](./03/serial.log) [💾](./03/registers.txt) |
| 4 | And the serial output should contain "Supervisor starting" | ✅ | 210ms | - [📜](./04/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[15053033788] [INFO] [kernel] [CPU0] BOOTFB: width=1920 height=1080 pitch=7680 bpp=32 format=Bgrx8888
[15060674327] [CONTRACT] [kernel] [CPU0] thing-os kernel starting...
[15064983805] [INFO] [bran::requests] [CPU0] Limine: Found 29 boot modules
[15067419645] [INFO] [bran::requests] [CPU0]   [0] /boot/sprout (cmdline='init') size=90640
[15069005841] [INFO] [bran::requests] [CPU0]   [1] /boot/bristle (cmdline='init') size=25008
[15069530232] [INFO] [bran::requests] [CPU0]   [2] /boot/rtc_cmos (cmdline='init') size=33368
[15070028788] [INFO] [bran::requests] [CPU0]   [3] /boot/clock (cmdline='') size=74256
[15070480088] [INFO] [bran::requests] [CPU0]   [4] /boot/ps2_kbd (cmdline='init') size=25008
[15071243195] [INFO] [bran::requests] [CPU0]   [5] /boot/echo (cmdline='init') size=29104
[15071671794] [INFO] [bran::requests] [CPU0]   [6] /boot/bloom (cmdline='init') size=928264
[15072093680] [INFO] [bran::requests] [CPU0]   [7] /boot/ps2_mouse (cmdline='init') size=29104
[15072661332] [INFO] [bran::requests] [CPU0]   [8] /boot/display_bootfb (cmdline='init') size=33368
[15073252726] [INFO] [bran::requests] [CPU0]   [9] /boot/display_virtio_gpu (cmdline='init') size=53864
[15073749319] [INFO] [bran::requests] [CPU0]   [10] /boot/fontd (cmdline='init') size=188944
[15074185521] [INFO] [bran::requests] [CPU0]   [11] /boot/blossom (cmdline='init') size=115216
[15074623701] [INFO] [bran::requests] [CPU0]   [12] /boot/flytrap (cmdline='') size=287256
[15075041767] [INFO] [bran::requests] [CPU0]   [13] /boot/virtio_netd (cmdline='') size=49680
[15075576421] [INFO] [bran::requests] [CPU0]   [14] /boot/netd (cmdline='') size=131600
[15076005344] [INFO] [bran::requests] [CPU0]   [15] /boot/fetchd (cmdline='') size=61968
[15076476497] [INFO] [bran::requests] [CPU0]   [16] /boot/anther (cmdline='') size=365072
[15076895305] [INFO] [bran::requests] [CPU0]   [17] /boot/photosynthesis (cmdline='') size=250464
[15077342050] [INFO] [bran::requests] [CPU0]   [18] /boot/ahci_disk (cmdline='') size=45664
[15077840317] [INFO] [bran::requests] [CPU0]   [19] /boot/iso9660d (cmdline='') size=49760
[15078277894] [INFO] [bran::requests] [CPU0]   [20] /boot/virtio_sound (cmdline='init') size=45488
[15078732642] [INFO] [bran::requests] [CPU0]   [21] /boot/beeper (cmdline='init') size=33296
[15079157504] [INFO] [bran::requests] [CPU0]   [22] /boot/nectar (cmdline='init') size=66064
[15079591963] [INFO] [bran::requests] [CPU0]   [23] /assets/wallpapers/leather.bmp (cmdline='') size=1179702
[15080330682] [INFO] [bran::requests] [CPU0]   [24] /assets/wallpapers/linen.bmp (cmdline='') size=4718646
[15080821401] [INFO] [bran::requests] [CPU0]   [25] /assets/fonts/NotoSans-Regular.ttf (cmdline='') size=569208
[15081327195] [INFO] [bran::requests] [CPU0]   [26] /assets/themes/genie_circles.wasm (cmdline='') size=2824
[15081819494] [INFO] [bran::requests] [CPU0]   [27] /assets/cursors/future/default.svg (cmdline='') size=3051
[15082390245] [INFO] [bran::requests] [CPU0]   [28] /boot/locale.conf (cmdline='') size=80
[15083686088] [INFO] [kernel::memory] [CPU0] Memory map has 64 entries
[15085119776] [INFO] [kernel::memory] [CPU0]   [0] 0x0 - 0xa0000 (Usable)
[15085913743] [INFO] [kernel::memory] [CPU0]   [1] 0x100000 - 0x800000 (Usable)
[15086287738] [INFO] [kernel::memory] [CPU0]   [2] 0x800000 - 0x808000 (Other)
[15086766455] [INFO] [kernel::memory] [CPU0]   [3] 0x808000 - 0x80b000 (Usable)
[15087143380] [INFO] [kernel::memory] [CPU0]   [4] 0x80b000 - 0x80c000 (Other)
[15087492311] [INFO] [kernel::memory] [CPU0]   [5] 0x80c000 - 0x811000 (Usable)
[15087846434] [INFO] [kernel::memory] [CPU0]   [6] 0x811000 - 0x900000 (Other)
[15088195087] [INFO] [kernel::memory] [CPU0]   [7] 0x900000 - 0x1780000 (Reserved)
[15088594376] [INFO] [kernel::memory] [CPU0]   [8] 0x1780000 - 0x786e1000 (Usable)
[15089178245] [INFO] [kernel::memory] [CPU0]   [9] 0x786e1000 - 0x78740000 (Reserved)
[15089590136] [INFO] [kernel::memory] [CPU0]   [10] 0x78740000 - 0x78741000 (Other)
[15089971551] [INFO] [kernel::memory] [CPU0]   [11] 0x78741000 - 0x78742000 (Reserved)
[15090363673] [INFO] [kernel::memory] [CPU0]   [12] 0x78742000 - 0x78743000 (Other)
[15090742573] [INFO] [kernel::memory] [CPU0]   [13] 0x78743000 - 0x78744000 (Reserved)
[15091133679] [INFO] [kernel::memory] [CPU0]   [14] 0x78744000 - 0x78745000 (Other)
[15091587969] [INFO] [kernel::memory] [CPU0]   [15] 0x78745000 - 0x78746000 (Reserved)
[15092004383] [INFO] [kernel::memory] [CPU0]   [16] 0x78746000 - 0x787d1000 (Other)
[15092385901] [INFO] [kernel::memory] [CPU0]   [17] 0x787d1000 - 0x787d2000 (Reserved)
[15092783085] [INFO] [kernel::memory] [CPU0]   [18] 0x787d2000 - 0x78c53000 (Other)
[15093165212] [INFO] [kernel::memory] [CPU0]   [19] 0x78c53000 - 0x78c54000 (Reserved)
[15093556908] [INFO] [kernel::memory] [CPU0]   [20] 0x78c54000 - 0x78d75000 (Other)
[15093991894] [INFO] [kernel::memory] [CPU0]   [21] 0x78d75000 - 0x78d76000 (Reserved)
[15094616926] [INFO] [kernel::memory] [CPU0]   [22] 0x78d76000 - 0x78d87000 (Other)
[15095007293] [INFO] [kernel::memory] [CPU0]   [23] 0x78d87000 - 0x78d89000 (Reserved)
[15095510962] [INFO] [kernel::memory] [CPU0]   [24] 0x78d89000 - 0x78d92000 (Other)
[15095985704] [INFO] [kernel::memory] [CPU0]   [25] 0x78d92000 - 0x78d94000 (Reserved)
[15096388343] [INFO] [kernel::memory] [CPU0]   [26] 0x78d94000 - 0x78da0000 (Other)
[15096758095] [INFO] [kernel::memory] [CPU0]   [27] 0x78da0000 - 0x78da2000 (Reserved)
[15097138134] [INFO] [kernel::memory] [CPU0]   [28] 0x78da2000 - 0x78daf000 (Other)
[15097504974] [INFO] [kernel::memory] [CPU0]   [29] 0x78daf000 - 0x78db0000 (Reserved)
[15097885137] [INFO] [kernel::memory] [CPU0]   [30] 0x78db0000 - 0x78dbc000 (Other)
[15098472801] [INFO] [kernel::memory] [CPU0]   [31] 0x78dbc000 - 0x78dbd000 (Reserved)
[15098894294] [INFO] [kernel::memory] [CPU0]   [32] 0x78dbd000 - 0x78dfb000 (Other)
[15099263020] [INFO] [kernel::memory] [CPU0]   [33] 0x78dfb000 - 0x78dfc000 (Reserved)
[15099646189] [INFO] [kernel::memory] [CPU0]   [34] 0x78dfc000 - 0x78e56000 (Other)
[15100015115] [INFO] [kernel::memory] [CPU0]   [35] 0x78e56000 - 0x78e57000 (Reserved)
[15100394094] [INFO] [kernel::memory] [CPU0]   [36] 0x78e57000 - 0x78e67000 (Other)
[15100852400] [INFO] [kernel::memory] [CPU0]   [37] 0x78e67000 - 0x78e68000 (Reserved)
[15101258635] [INFO] [kernel::memory] [CPU0]   [38] 0x78e68000 - 0x78e89000 (Other)
[15101640326] [INFO] [kernel::memory] [CPU0]   [39] 0x78e89000 - 0x78e8a000 (Reserved)
[15102031525] [INFO] [kernel::memory] [CPU0]   [40] 0x78e8a000 - 0x78e97000 (Other)
[15102418459] [INFO] [kernel::memory] [CPU0]   [41] 0x78e97000 - 0x78e98000 (Reserved)
[15102868616] [INFO] [kernel::memory] [CPU0]   [42] 0x78e98000 - 0x78edf000 (Other)
[15103266251] [INFO] [kernel::memory] [CPU0]   [43] 0x78edf000 - 0x78ee0000 (Reserved)
[15103647203] [INFO] [kernel::memory] [CPU0]   [44] 0x78ee0000 - 0x78efd000 (Other)
[15104016350] [INFO] [kernel::memory] [CPU0]   [45] 0x78efd000 - 0x78efe000 (Reserved)
[15104400006] [INFO] [kernel::memory] [CPU0]   [46] 0x78efe000 - 0x78f2d000 (Other)
[15104769804] [INFO] [kernel::memory] [CPU0]   [47] 0x78f2d000 - 0x78f3b000 (Other)
[15105295446] [INFO] [kernel::memory] [CPU0]   [48] 0x78f3b000 - 0x78f44000 (Other)
[15105705453] [INFO] [kernel::memory] [CPU0]   [49] 0x78f44000 - 0x79027000 (Other)
[15106085188] [INFO] [kernel::memory] [CPU0]   [50] 0x79027000 - 0x79249000 (Other)
[15106464388] [INFO] [kernel::memory] [CPU0]   [51] 0x79249000 - 0x7a16c000 (Reserved)
[15106859536] [INFO] [kernel::memory] [CPU0]   [52] 0x7a16c000 - 0x7bb6c000 (Usable)
[15107242963] [INFO] [kernel::memory] [CPU0]   [53] 0x7bb6c000 - 0x7bb93000 (Reserved)
[15107859595] [INFO] [kernel::memory] [CPU0]   [54] 0x7bb93000 - 0x7bb9b000 (Other)
[15108246909] [INFO] [kernel::memory] [CPU0]   [55] 0x7bb9b000 - 0x7bb9f000 (Reserved)
[15108640670] [INFO] [kernel::memory] [CPU0]   [56] 0x7bb9f000 - 0x7bba7000 (Other)
[15109027155] [INFO] [kernel::memory] [CPU0]   [57] 0x7bba7000 - 0x7bba9000 (Reserved)
[15109421299] [INFO] [kernel::memory] [CPU0]   [58] 0x7bba9000 - 0x7bbb0000 (Other)
[15109878429] [INFO] [kernel::memory] [CPU0]   [59] 0x7bbb0000 - 0x7bbc3000 (Other)
[15110279537] [INFO] [kernel::memory] [CPU0]   [60] 0x7bbc3000 - 0x7bbcc000 (Other)
[15110665077] [INFO] [kernel::memory] [CPU0]   [61] 0x7bbcc000 - 0x7bbd3000 (Other)
[15111047942] [INFO] [kernel::memory] [CPU0]   [62] 0x7bbd3000 - 0x7bbea000 (Other)
[15111431812] [INFO] [kernel::memory] [CPU0]   [63] 0x7bbea000 - 0x7bc0a000 (Reserved)
[15112188431] [INFO] [kernel::memory] [CPU0] HHDM Offset: 0xffff800000000000
[15403826805] [CONTRACT] [kernel::memory] [CPU0] Frame allocator initialized with 495865 free frames
[15416917237] [INFO] [bran::arch] [CPU0] IOAPIC: hhdm=0xffff800000000000
[15423652356] [INFO] [bran::arch] [CPU0] IOAPIC: Disabling legacy PIC...
[15425117882] [INFO] [bran::arch] [CPU0] IOAPIC: PIC disabled OK
[15426032398] [INFO] [bran::arch] [CPU0] IOAPIC: RSDP virt=0x7f77e014
[15430992141] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: total length 120
[15433077165] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 0, len 8 at 0xffff80007f77802c
[15434119380] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 1, len 12 at 0xffff80007f778034
[15434947637] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778040
[15435712779] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f77804a
[15436162946] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778054
[15436706719] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f77805e
[15437166248] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 2, len 10 at 0xffff80007f778068
[15437616934] [INFO] [bran::arch::x86_64::acpi] [CPU0] MADT: Entry type 4, len 6 at 0xffff80007f778072
[15438738875] [INFO] [bran::arch] [CPU0] IOAPIC: MADT parsed OK
[15439815922] [INFO] [bran::arch] [CPU0] SMP: Found 1 CPUs (CPU_COUNT now = 1)
[15440543835] [INFO] [bran::arch] [CPU0] IOAPIC: Found at phys 0xfec00000, GSI base 0
[15442474021] [INFO] [bran::arch] [CPU0] IOAPIC: Registers initialized
[15443683361] [INFO] [bran::arch] [CPU0] IOAPIC: version 0x20, 24 redir entries
[15445500363] [INFO] [bran::arch] [CPU0] IOAPIC: All pins masked
[15447187033] [INFO] [bran::arch] [CPU0] IOAPIC: IRQ1 -> GSI 1 -> 0x21
[15448171589] [INFO] [bran::arch] [CPU0] IOAPIC: IRQ12 -> GSI 12 -> 0x2C
[15448770382] [INFO] [bran::arch] [CPU0] IOAPIC: Init complete
[15449540597] [CONTRACT] [kernel] [CPU0] Initializing global allocator...
[15993675983] [INFO] [kernel::memory::global_alloc] [CPU0] Global allocator initialized (LinkedHeap, 32MB)
[15994544192] [CONTRACT] [kernel] [CPU0] Initializing SIMD...
[15996279183] [CONTRACT] [kernel] [CPU0] Initializing tasking...
[16003591084] [INFO] [kernel::task::scheduler] [CPU0]   Acquiring scheduler lock...
[16005467553] [INFO] [kernel::task::scheduler] [CPU0]   Lock acquired, checking if initialized...
[16006254180] [INFO] [kernel::task::scheduler] [CPU0]   Allocating scheduler...
[16014287369] [INFO] [kernel::task::scheduler] [CPU0]   Leaking scheduler...
[16014902940] [INFO] [kernel::task::scheduler] [CPU0]   Initializing boot task...
[16019188374] [INFO] [kernel::task::scheduler] [CPU0]   Creating boot task...
[16027729188] [INFO] [kernel::task::scheduler] [CPU0]   Creating idle tasks...
[16030121212] [DEBUG] [kernel::task::scheduler::spawn] [CPU0] SCHED: Task 1 assigned to CPU 0
[16048405330] [INFO] [kernel::task::scheduler] [CPU0]   Creating graph worker task...
[16049179376] [DEBUG] [kernel::task::scheduler::spawn] [CPU0] SCHED: Task 2 assigned to CPU 0
[16051196605] [INFO] [kernel::task::scheduler] [CPU0]   Boot task initialized
[16051740999] [INFO] [kernel::task::scheduler] [CPU0]   Storing scheduler pointer...
[16052773287] [CONTRACT] [kernel::task::scheduler] [CPU0] Scheduler initialized
[16053667284] [INFO] [kernel] [CPU0] Kernel: Detected 1 CPUs. SMP will be brought up lazily.
[16061879210] [INFO] [kernel::root] [CPU0] Spawning Root service...
[16063043026] [DEBUG] [kernel::task::scheduler::spawn] [CPU0] SCHED: Task 3 assigned to CPU 0
[16064824116] [INFO] [kernel::root::boot_register] [CPU0] ROOT: boot registration begin (Census Phase 1 v0.2)
[16065493191] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: registering Host...
[16081091267] [INFO] [kernel::root::service] [CPU0] ROOT: started once
[16081703036] [CONTRACT] [kernel::root::service] [CPU0] ROOT: Initializing components...
[16083001783] [CONTRACT] [kernel::root::service] [CPU0] ROOT: Graph initialized
[16083836501] [CONTRACT] [kernel::root::service] [CPU0] ROOT: Journal initialized
[16084506272] [CONTRACT] [kernel::root::service] [CPU0] ROOT: Interner initialized
[16095990521] [CONTRACT] [kernel::root::service] [CPU0] ROOT: LogSymbols initialized
[16099768751] [CONTRACT] [kernel::root::service] [CPU0] ROOT: BatchScratch initialized
[16100315608] [CONTRACT] [kernel::root::service] [CPU0] ROOT: Entering main loop
[16153966307] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: Host registered: t1
[16653566982] [CONTRACT] [alloc] [CPU0] MEDIUM ALLOC #1: 120 KB align=8 total=0MB
[17140041883] [CONTRACT] [alloc] [CPU0] MEDIUM ALLOC #2: 240 KB align=8 total=0MB
[17876554517] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: Census Phase 2: PCI
[17877493291] [INFO] [kernel::root::pci] [CPU0] PCI: Starting enumeration...
[17978870108] [INFO] [kernel::root::pci] [CPU0] PCI: 00:00.0 8086:29c0 Intel Corporation 82G33/G31/P35/P31 Express DRAM Controller class=06:00 prog_if=00 rev=00
[18026919863] [INFO] [kernel::root::pci] [CPU0] PCI: 00:01.0 1234:1111 (unknown vendor) (unknown device) class=03:00 prog_if=00 rev=02
[18039443589] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR0: phys=0x80000000 size=0x1000000
[18040465917] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR2: phys=0x810c5000 size=0x1000
[18100045827] [INFO] [kernel::root::pci] [CPU0] PCI: 00:02.0 8086:10d3 Intel Corporation 82574L Gigabit Network Connection class=02:00 prog_if=00 rev=00
[18118112559] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR0: phys=0x810a0000 size=0x20000
[18118781712] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR1: phys=0x81080000 size=0x20000
[18119233187] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR3: phys=0x810c0000 size=0x4000
[18190747475] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.0 8086:2918 Intel Corporation 82801IB (ICH9) LPC Interface Controller class=06:01 prog_if=00 rev=02
[18192854065] [INFO] [kernel::root::pci] [CPU0] PCI: Found LPC/ISA bridge at 00:1f.0
[18285323002] [INFO] [kernel::root::pci] [CPU0] LPC: Created Legacy IO bus with CMOS and PS/2 controller
[18350315984] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.2 8086:2922 Intel Corporation 82801IR/IO/IH (ICH9R/DO/DH) 6 port SATA Controller [AHCI mode] class=01:06 prog_if=01 rev=02
[18358446418] [INFO] [kernel::root::pci] [CPU0] PCI:   BAR5: phys=0x810c4000 size=0x1000
[18362337191] [INFO] [kernel::root::pci] [CPU0] PCI: Found AHCI SATA controller at 00:1f.2
[18365622901] [INFO] [kernel::root::pci] [CPU0] PCI: Registered AHCI controller (graph_id=174, idx=3) BAR5=0x810c4000
[18431349428] [INFO] [kernel::root::pci] [CPU0] PCI: 00:1f.3 8086:2930 Intel Corporation 82801I (ICH9 Family) SMBus Controller class=0c:05 prog_if=00 rev=02
[18434618102] [CONTRACT] [kernel::root::boot_register] [CPU0] ROOT: registered items. host=t1 kernel=t3
[18435685641] [CONTRACT] [kernel] [CPU0] KERNEL: root census complete: host=t1 kernel=t3 root=t4
[18436443693] [CONTRACT] [kernel] [CPU0] Kernel: Enumerating 29 boot modules...
[18437577301] [CONTRACT] [kernel] [CPU0]   [0] name='/boot/sprout' cmdline='init' size=90640
[18438195075] [CONTRACT] [kernel] [CPU0]   [1] name='/boot/bristle' cmdline='init' size=25008
[18438721374] [CONTRACT] [kernel] [CPU0]   [2] name='/boot/rtc_cmos' cmdline='init' size=33368
[18439165959] [CONTRACT] [kernel] [CPU0]   [3] name='/boot/clock' cmdline='' size=74256
[18439588327] [CONTRACT] [kernel] [CPU0]   [4] name='/boot/ps2_kbd' cmdline='init' size=25008
[18440166777] [CONTRACT] [kernel] [CPU0]   [5] name='/boot/echo' cmdline='init' size=29104
[18440642048] [CONTRACT] [kernel] [CPU0]   [6] name='/boot/bloom' cmdline='init' size=928264
[18441089521] [CONTRACT] [kernel] [CPU0]   [7] name='/boot/ps2_mouse' cmdline='init' size=29104
[18441539764] [CONTRACT] [kernel] [CPU0]   [8] name='/boot/display_bootfb' cmdline='init' size=33368
[18442032474] [CONTRACT] [kernel] [CPU0]   [9] name='/boot/display_virtio_gpu' cmdline='init' size=53864
[18442617020] [CONTRACT] [kernel] [CPU0]   [10] name='/boot/fontd' cmdline='init' size=188944
[18443331780] [CONTRACT] [kernel] [CPU0]   [11] name='/boot/blossom' cmdline='init' size=115216
[18443809820] [CONTRACT] [kernel] [CPU0]   [12] name='/boot/flytrap' cmdline='' size=287256
[18444250220] [CONTRACT] [kernel] [CPU0]   [13] name='/boot/virtio_netd' cmdline='' size=49680
[18444793721] [CONTRACT] [kernel] [CPU0]   [14] name='/boot/netd' cmdline='' size=131600
[18445240111] [CONTRACT] [kernel] [CPU0]   [15] name='/boot/fetchd' cmdline='' size=61968
[18445682944] [CONTRACT] [kernel] [CPU0]   [16] name='/boot/anther' cmdline='' size=365072
[18446134688] [CONTRACT] [kernel] [CPU0]   [17] name='/boot/photosynthesis' cmdline='' size=250464
[18446861752] [CONTRACT] [kernel] [CPU0]   [18] name='/boot/ahci_disk' cmdline='' size=45664
[18447328631] [CONTRACT] [kernel] [CPU0]   [19] name='/boot/iso9660d' cmdline='' size=49760
[18447775316] [CONTRACT] [kernel] [CPU0]   [20] name='/boot/virtio_sound' cmdline='init' size=45488
[18448263960] [CONTRACT] [kernel] [CPU0]   [21] name='/boot/beeper' cmdline='init' size=33296
[18448731286] [CONTRACT] [kernel] [CPU0]   [22] name='/boot/nectar' cmdline='init' size=66064
[18449276245] [CONTRACT] [kernel] [CPU0]   [23] name='/assets/wallpapers/leather.bmp' cmdline='' size=1179702
[18449856460] [CONTRACT] [kernel] [CPU0]   [24] name='/assets/wallpapers/linen.bmp' cmdline='' size=4718646
[18450377113] [CONTRACT] [kernel] [CPU0]   [25] name='/assets/fonts/NotoSans-Regular.ttf' cmdline='' size=569208
[18450901855] [CONTRACT] [kernel] [CPU0]   [26] name='/assets/themes/genie_circles.wasm' cmdline='' size=2824
[18451508286] [CONTRACT] [kernel] [CPU0]   [27] name='/assets/cursors/future/default.svg' cmdline='' size=3051
[18452032221] [CONTRACT] [kernel] [CPU0]   [28] name='/boot/locale.conf' cmdline='' size=80
[18453526002] [INFO] [kernel] [CPU0] Found init module: /boot/sprout (cmdline: 'init'), loading...
[18521257835] [INFO] [kernel::task::loader] [CPU0] Loading module: /boot/sprout
[18522807651] [DEBUG] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[18536693415] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[18555065701] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=210000 exec=false
[18559839965] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=215000 exec=false
[18571900106] [INFO] [kernel] [CPU0] Spawning sprout with registry at 0x600000...
[18572598284] [CONTRACT] [kernel] [CPU0] Spawning init process...
[18574639095] [DEBUG] [kernel::task::scheduler::spawn] [CPU0] SCHED: Task 4 (user task/process) assigned to CPU 0
[18577557354] [INFO] [kernel] [CPU0] System initialized. Setting up preemption timer (100Hz)...
[18603435704] [INFO] [bran::arch::x86_64] [CPU0] LAPIC: calibrated timer (62291900 ticks/sec), init_cnt=622919 for 100Hz
[18604672787] [CONTRACT] [kernel] [CPU0] Entering scheduler loop.
[18609544209] [DEBUG] [kernel::task::scheduler::spawn] [CPU0] Trampoline entered. Arg: 0xffffffffb0004608
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600000
[18615884572] [INFO] [bran::arch::x86_64::enter_user] [CPU0] Entering user mode tid=4 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562158925 RSP_BEFORE=18446744072367889168 RFLAGS_BEFORE=130 CR3_BEFORE=50331648 fs_base=0 gs_base=18446744071564147976
[18670797712] [INFO] [sprout] [CPU0] [sprout] whoami: cs=0x2b ss=0x23 cpl=3 rsp=0x7ff9b0 rip=0x2013ff rflags=0x202
[18682314042] [INFO] [sprout] [CPU0] SPROUT: v0.4 starting (Supervisor Mode)...
[18683506107] [INFO] [sprout::devtree] [CPU0] SPROUT: devtree::init entry (v0.2)
[18684582832] [INFO] [sprout::devtree] [CPU0] SPROUT: Step 1: Find Host
[18698973169] [INFO] [sprout::devtree] [CPU0] SPROUT: Step 2: HHDM
[18707352356] [INFO] [sprout::devtree] [CPU0] SPROUT: Step 3: Platform Bus
[18712711261] [INFO] [sprout::devtree] [CPU0] SPROUT: Step 4: Firmware
[18713870932] [INFO] [sprout::devtree] [CPU0] SPROUT: Finding ACPI...
[18719860416] [INFO] [sprout::devtree] [CPU0] SPROUT: Found 1 ACPI nodes
[18725307249] [INFO] [sprout::devtree] [CPU0] SPROUT: ACPI RSDP = 0x7f77e014
[18726296391] [INFO] [sprout::devtree] [CPU0] SPROUT: Finding DTB...
[18731539202] [INFO] [sprout::devtree] [CPU0] SPROUT: Found 0 DTB nodes
[18732693651] [INFO] [sprout::devtree] [CPU0] SPROUT: Init OK, returning context
[18733603047] [INFO] [sprout::devtree] [CPU0] SPROUT: build() called
[18734639542] [INFO] [sprout::devtree::x86_64] [CPU0] SPROUT: x86_64 platform enrichment... (v0.2)
[18739882652] [INFO] [sprout::devtree::x86_64] [CPU0] SPROUT: x86_64 enumerate done
[18741334495] [INFO] [sprout] [CPU0] SPROUT: About to create Supervisor...
[18742524662] [INFO] [sprout] [CPU0] SPROUT: Supervisor created, calling run_forever...
[18743655970] [INFO] [sprout::supervisor] [CPU0] SPROUT: Supervisor starting (minimal mode)...
[18744651830] [INFO] [sprout::supervisor] [CPU0] SPROUT: Discovering modules...
[18751736724] [INFO] [sprout::supervisor] [CPU0] SPROUT: Found 29 modules
[18843732585] [INFO] [sprout::supervisor] [CPU0] SPROUT: Module[0] = '/boot/sprout'
[18854467652] [INFO] [sprout::supervisor] [CPU0] SPROUT: Module[1] = '/boot/bristle'
[18866865494] [INFO] [sprout::supervisor] [CPU0] SPROUT: Module[2] = '/boot/rtc_cmos'
[18875325600] [INFO] [sprout::supervisor] [CPU0] SPROUT: Module[3] = '/boot/clock'
[18880170139] [INFO] [sprout::supervisor] [CPU0] SPROUT: Discovered app: /boot/clock
[18889508897] [INFO] [sprout::supervisor] [CPU0] SPROUT: Module[4] = '/boot/ps2_kbd'
[18898295295] [INFO] [sprout::supervisor] [CPU0] SPROUT: Module[5] = '/boot/echo'
[18906544459] [INFO] [sprout::supervisor] [CPU0] SPROUT: Module[6] = '/boot/bloom'
[18915359990] [INFO] [sprout::supervisor] [CPU0] SPROUT: Module[7] = '/boot/ps2_mouse'
[18922938552] [INFO] [sprout::supervisor] [CPU0] SPROUT: Module[8] = '/boot/display_bootfb'
[19110124783] [INFO] [sprout::supervisor] [CPU0] SPROUT: Module[9] = '/boot/display_virtio_gpu'
[19118656605] [INFO] [sprout::supervisor] [CPU0] SPROUT: Module[10] = '/boot/fontd'
[19126121322] [INFO] [sprout::supervisor] [CPU0] SPROUT: Module[11] = '/boot/blossom'
[19135020295] [INFO] [sprout::supervisor] [CPU0] SPROUT: Module[12] = '/boot/flytrap'
[19142974431] [INFO] [sprout::supervisor] [CPU0] SPROUT: Module[13] = '/boot/virtio_netd'
[19151246845] [INFO] [sprout::supervisor] [CPU0] SPROUT: Module[14] = '/boot/netd'
[19159247268] [INFO] [sprout::supervisor] [CPU0] SPROUT: Module[15] = '/boot/fetchd'
[19166659089] [INFO] [sprout::supervisor] [CPU0] SPROUT: Module[16] = '/boot/anther'
[19174245332] [INFO] [sprout::supervisor] [CPU0] SPROUT: Module[17] = '/boot/photosynthesis'
[19182044623] [INFO] [sprout::supervisor] [CPU0] SPROUT: Module[18] = '/boot/ahci_disk'
[19190357146] [INFO] [sprout::supervisor] [CPU0] SPROUT: Module[19] = '/boot/iso9660d'
[19197693999] [INFO] [sprout::supervisor] [CPU0] SPROUT: Module[20] = '/boot/virtio_sound'
[19205717348] [INFO] [sprout::supervisor] [CPU0] SPROUT: Module[21] = '/boot/beeper'
[19213293341] [INFO] [sprout::supervisor] [CPU0] SPROUT: Module[22] = '/boot/nectar'
[19220793877] [INFO] [sprout::supervisor] [CPU0] SPROUT: Module[23] = '/assets/wallpapers/leather.bmp'
[19229531961] [INFO] [sprout::supervisor] [CPU0] SPROUT: Module[24] = '/assets/wallpapers/linen.bmp'
[19236920010] [INFO] [sprout::supervisor] [CPU0] SPROUT: Module[25] = '/assets/fonts/NotoSans-Regular.ttf'
[19244480354] [INFO] [sprout::supervisor] [CPU0] SPROUT: Module[26] = '/assets/themes/genie_circles.wasm'
[19252508222] [INFO] [sprout::supervisor] [CPU0] SPROUT: Module[27] = '/assets/cursors/future/default.svg'
[19259999175] [INFO] [sprout::supervisor] [CPU0] SPROUT: Module[28] = '/boot/locale.conf'
[19264426891] [INFO] [sprout::registry] [CPU0] SPROUT: Scanning boot modules...
[19307941831] [INFO] [sprout::registry] [CPU0] SPROUT: Registering driver 'dev.rtc.Cmos' -> '/boot/rtc_cmos' (fallback)
[19495292276] [INFO] [sprout::registry] [CPU0] SPROUT: Registry scan complete. Found 1 drivers.
[19496452525] [INFO] [sprout::pipelines] [CPU0] SPROUT: Setting up audio pipeline...
[19502189004] [INFO] [sprout::pipelines] [CPU0] SPROUT: No Sound device found
[19503185151] [INFO] [sprout::pipelines] [CPU0] SPROUT: Setting up display pipeline...
[19530558154] [INFO] [sprout::pipelines] [CPU0] SPROUT: Using boot framebuffer (fallback)
[19532402417] [INFO] [sprout::pipelines] [CPU0] SPROUT: Display backend: BootFB (1920x1080 stride=7680)
[20411113567] [INFO] [kernel::task::loader] [CPU0] Loading module: /boot/display_bootfb
[20411810609] [DEBUG] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[20412984788] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[20419299831] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=205000 exec=false
[20421247960] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=207000 exec=false
[20431046258] [DEBUG] [kernel::task::scheduler::spawn] [CPU0] SCHED: Task 5 (user task/process) assigned to CPU 0
[20433882412] [INFO] [sprout::pipelines] [CPU0] SPROUT: Spawned display driver '/display_bootfb' (PID=5)
[20447939504] [DEBUG] [kernel::task::scheduler::spawn] [CPU0] Trampoline entered. Arg: 0xffffffffb0004608
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x30002
[20449151362] [INFO] [bran::arch::x86_64::enter_user] [CPU0] Entering user mode tid=5 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562158925 RSP_BEFORE=18446744072368266384 RFLAGS_BEFORE=134 CR3_BEFORE=59072512 fs_base=0 gs_base=18446744071564147976
[20454012322] [INFO] [display_bootfb] [CPU0] display_bootfb: starting (drv_req_r=2, drv_resp_w=3)
[20484997727] [INFO] [sprout::pipelines] [CPU0] SPROUT: Setting up input pipeline (keyboard + mouse)...
[20487453140] [INFO] [sprout::pipelines] [CPU0] SPROUT: Created kbd_raw port (w=5, r=6)
[20489428083] [INFO] [sprout::pipelines] [CPU0] SPROUT: Created mouse_raw port (w=7, r=8)
[20490888305] [INFO] [kernel::task::loader] [CPU0] Loading module: /boot/ps2_kbd
[20491486528] [DEBUG] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20492589259] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[20497694313] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=204000 exec=false
[20499235981] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=205000 exec=false
[20507923646] [DEBUG] [kernel::task::scheduler::spawn] [CPU0] SCHED: Task 6 (user task/process) assigned to CPU 0
[20509874824] [INFO] [sprout::pipelines] [CPU0] SPROUT: Spawned ps2_kbd (PID=6)
[20511811678] [INFO] [kernel::task::loader] [CPU0] Loading module: /boot/ps2_mouse
[20512429872] [DEBUG] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20513580241] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[20518688482] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=204000 exec=false
[20520486609] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=206000 exec=false
[20528300198] [DEBUG] [kernel::task::scheduler::spawn] [CPU0] SCHED: Task 7 (user task/process) assigned to CPU 0
[20530455617] [INFO] [sprout::pipelines] [CPU0] SPROUT: Spawned ps2_mouse (PID=7)
[20534366120] [INFO] [kernel::task::loader] [CPU0] Loading module: /boot/bristle
[20535547359] [DEBUG] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[20536813534] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[20542185235] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=204000 exec=false
[20543408869] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=205000 exec=false
[20551114188] [DEBUG] [kernel::task::scheduler::spawn] [CPU0] SCHED: Task 8 (user task/process) assigned to CPU 0
[20553944487] [INFO] [sprout::pipelines] [CPU0] SPROUT: Spawned bristle (PID=8)
[20572966043] [INFO] [kernel::syscall::handlers::device] [CPU0] DEVICE: task 5 claimed device 157 (handle 0)
[20594886200] [INFO] [kernel::syscall::handlers::device] [CPU0] DEVICE: Mapped BAR0 phys=0x80000000 size=0x7e9000 -> virt=0x10000000
[21367204436] [DEBUG] [kernel::task::scheduler::spawn] [CPU0] Trampoline entered. Arg: 0xffffffffb0004608
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x5
[21368675141] [INFO] [bran::arch::x86_64::enter_user] [CPU0] Entering user mode tid=6 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562158925 RSP_BEFORE=18446744072368331920 RFLAGS_BEFORE=134 CR3_BEFORE=59191296 fs_base=0 gs_base=18446744071564147976
[21373944347] [INFO] [ps2_kbd] [CPU0] ps2_kbd: online (handle=5)
[21377584737] [DEBUG] [kernel::task::scheduler::spawn] [CPU0] Trampoline entered. Arg: 0xffffffffb0046a98
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x7
[21378858525] [INFO] [bran::arch::x86_64::enter_user] [CPU0] Entering user mode tid=7 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562158925 RSP_BEFORE=18446744072368397456 RFLAGS_BEFORE=134 CR3_BEFORE=59301888 fs_base=0 gs_base=18446744071564147976
[21384062630] [INFO] [ps2_mouse] [CPU0] ps2_mouse: online (handle=7)
[21385144438] [INFO] [ps2_mouse] [CPU0] ps2_mouse: starting robust init
[21389577611] [DEBUG] [kernel::task::scheduler::spawn] [CPU0] Trampoline entered. Arg: 0xffffffffb00491e8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600080009000b
[21391065841] [INFO] [bran::arch::x86_64::enter_user] [CPU0] Entering user mode tid=8 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562158925 RSP_BEFORE=18446744072368479376 RFLAGS_BEFORE=134 CR3_BEFORE=59416576 fs_base=0 gs_base=18446744071564147976
[21395751039] [INFO] [bristle] [CPU0] bristle: online (kbd=6, mouse=8, evt=9, echo=11)
[21411810986] [INFO] [ps2_kbd] [CPU0] ps2_kbd: created driver node 185
[21414306033] [INFO] [kernel::syscall::handlers::device] [CPU0] DEVICE: task subscribed to vector 0x21
[21415860481] [INFO] [ps2_kbd] [CPU0] ps2_kbd: subscribed to IRQ1 (vector 0x21)
[21443419874] [INFO] [display_bootfb] [CPU0] display_bootfb: created back buffer (size=8294400)
[21452303410] [INFO] [sprout::pipelines] [CPU0] SPROUT: Writing bloom BS: drv_req=1, drv_resp=4, bristle_evt=10
[21460722326] [INFO] [ps2_kbd] [CPU0] ps2_kbd: entering interrupt-driven loop
[21469847548] [INFO] [sprout::pipelines] [CPU0] SPROUT: Bloom handles via BS=181 backend=BootFB
[21471467793] [INFO] [kernel::task::loader] [CPU0] Loading module: /boot/bloom
[21471980353] [DEBUG] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21473455794] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[21649721780] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=2cd000 exec=false
[21667551790] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=2e1000 exec=false
[21676110471] [DEBUG] [kernel::task::scheduler::spawn] [CPU0] SCHED: Task 9 (user task/process) assigned to CPU 0
[21679897144] [INFO] [sprout::pipelines] [CPU0] SPROUT: Spawned bloom (PID=9)
[21683778108] [INFO] [kernel::task::loader] [CPU0] Loading module: /boot/echo
[21684302650] [DEBUG] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21685475854] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[21690394787] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=204000 exec=false
[21692300890] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=206000 exec=false
[21701485251] [DEBUG] [kernel::task::scheduler::spawn] [CPU0] SCHED: Task 10 (user task/process) assigned to CPU 0
[21703493405] [INFO] [sprout::pipelines] [CPU0] SPROUT: Spawned echo (PID=10)
[21704969480] [INFO] [sprout::pipelines] [CPU0] SPROUT: Input pipeline ready (keyboard + mouse)
[21706112090] [INFO] [sprout::supervisor] [CPU0] SPROUT: spawn_apps start. tasks len=7
[21708946673] [INFO] [sprout::supervisor] [CPU0] SPROUT: Adding fallback app '/boot/flytrap'
[21710661676] [INFO] [sprout::supervisor] [CPU0] SPROUT: Adding fallback app '/boot/fontd'
[21711894677] [INFO] [sprout::supervisor] [CPU0] SPROUT: Adding fallback app '/boot/blossom'
[21713022335] [INFO] [sprout::supervisor] [CPU0] SPROUT: Adding fallback app '/boot/cambium'
[21714007142] [INFO] [sprout::supervisor] [CPU0] SPROUT: Adding fallback app '/boot/ahci_disk'
[21715241063] [INFO] [sprout::supervisor] [CPU0] SPROUT: Adding fallback app '/boot/iso_reader'
[21716239836] [INFO] [sprout::supervisor] [CPU0] SPROUT: Adding fallback app '/boot/font_explorer'
[21717344428] [INFO] [sprout::supervisor] [CPU0] SPROUT: Adding fallback app '/boot/photosynthesis'
[21718356842] [INFO] [sprout::supervisor] [CPU0] SPROUT: Adding fallback app '/boot/nectar'
[21719526297] [INFO] [sprout::supervisor] [CPU0] SPROUT: Adding fallback app '/boot/fetchd'
[21720827521] [INFO] [sprout::supervisor] [CPU0] SPROUT: Launching app '/boot/clock'
[21722436715] [INFO] [kernel::task::loader] [CPU0] Loading module: /boot/clock
[21722947974] [DEBUG] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21723975175] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[21738714536] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=20d000 exec=false
[21742945812] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=211000 exec=false
[21753331439] [DEBUG] [kernel::task::scheduler::spawn] [CPU0] SCHED: Task 11 (user task/process) assigned to CPU 0
[21755278666] [INFO] [sprout::supervisor] [CPU0] SPROUT: App launched (PID=11)
[21756917641] [INFO] [sprout::supervisor] [CPU0] SPROUT: Launching app '/boot/flytrap'
[21758565428] [INFO] [kernel::task::loader] [CPU0] Loading module: /boot/flytrap
[21759087383] [DEBUG] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[21760076966] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[21814994791] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=237000 exec=false
[21827363317] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=245000 exec=false
[21835699420] [DEBUG] [kernel::task::scheduler::spawn] [CPU0] SCHED: Task 12 (user task/process) assigned to CPU 0
[21837565118] [INFO] [sprout::supervisor] [CPU0] SPROUT: App launched (PID=12)
[21838659475] [INFO] [sprout::supervisor] [CPU0] SPROUT: Seeding initial asset requests...
[21847280578] [INFO] [ps2_mouse] [CPU0] ps2_mouse: sending reset (0xFF)
[21850979536] [INFO] [ps2_mouse] [CPU0] ps2_mouse: reset ACK received (0xfa)
[21852018749] [INFO] [ps2_mouse] [CPU0] ps2_mouse: BAT result received (0xaa)
[21853611608] [INFO] [ps2_mouse] [CPU0] ps2_mouse: device ID received (0x00)
[21854389976] [INFO] [ps2_mouse] [CPU0] ps2_mouse: sending set defaults (0xF6)
[21855494172] [INFO] [ps2_mouse] [CPU0] ps2_mouse: setting sample rate (100)
[21856615840] [INFO] [ps2_mouse] [CPU0] ps2_mouse: setting resolution (3)
[21860633638] [INFO] [ps2_mouse] [CPU0] ps2_mouse: sending enable (0xF4)
[21861686555] [INFO] [ps2_mouse] [CPU0] ps2_mouse: enable ACK received (0xfa)
[21862606340] [INFO] [ps2_mouse] [CPU0] ps2_mouse: init done
[21863259169] [INFO] [kernel::syscall::handlers::device] [CPU0] DEVICE: task subscribed to vector 0x2c
[21864102957] [INFO] [ps2_mouse] [CPU0] ps2_mouse: subscribed to IRQ12 (vector 0x2c)
[21865054072] [INFO] [ps2_mouse] [CPU0] ps2_mouse: entering interrupt-driven loop
[21866090617] [DEBUG] [kernel::task::scheduler::spawn] [CPU0] Trampoline entered. Arg: 0xffffffffb0004608
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xb5
[21867321256] [INFO] [bran::arch::x86_64::enter_user] [CPU0] Entering user mode tid=9 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562158925 RSP_BEFORE=18446744072368544912 RFLAGS_BEFORE=134 CR3_BEFORE=67862528 fs_base=0 gs_base=18446744071564147976
[21872344094] [INFO] [bloom::logging] [CPU0] bloom: logging initialized
[21911454041] [INFO] [bloom] [CPU0] bloom: [bloom] SIMD backend: SSE2 (x86_64)
[21914080179] [DEBUG] [kernel::task::scheduler::spawn] [CPU0] Trampoline entered. Arg: 0xffffffffb0046a98
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xc
[21916135306] [INFO] [bran::arch::x86_64::enter_user] [CPU0] Entering user mode tid=10 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562158925 RSP_BEFORE=18446744072368610448 RFLAGS_BEFORE=134 CR3_BEFORE=68874240 fs_base=0 gs_base=18446744071564147976
[21920759153] [INFO] [echo] [CPU0] echo: starting up
[21921955930] [INFO] [echo] [CPU0] echo: ready for Bristle events (keyboard + mouse)
[21924314450] [DEBUG] [kernel::task::scheduler::spawn] [CPU0] Trampoline entered. Arg: 0xffffffffb00491e8
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[21925537038] [INFO] [bran::arch::x86_64::enter_user] [CPU0] Entering user mode tid=11 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562158925 RSP_BEFORE=18446744072368675984 RFLAGS_BEFORE=134 CR3_BEFORE=68988928 fs_base=0 gs_base=18446744071564147976
[21931509898] [DEBUG] [kernel::task::scheduler::spawn] [CPU0] Trampoline entered. Arg: 0xffffffffb00493a0
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[21932563455] [INFO] [bran::arch::x86_64::enter_user] [CPU0] Entering user mode tid=12 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562158925 RSP_BEFORE=18446744072368741520 RFLAGS_BEFORE=134 CR3_BEFORE=69148672 fs_base=0 gs_base=18446744071564147976
[21937095114] [INFO] [flytrap] [CPU0] FLYTRAP: Starting unified content provider service...
[21938262789] [INFO] [flytrap] [CPU0] FLYTRAP: Service contract validated - graph-native asset watcher
[21939278947] [INFO] [flytrap] [CPU0] FLYTRAP: Initializing Limine module content source...
[21965568579] [INFO] [bloom] [CPU0] [bloom] EARLY boot args: bristle_evt=10 arg_req=1 arg_resp=4
[22100736613] [INFO] [bloom::compositor] [CPU0] bloom: compositor bytespace 177 (1920x1080 stride=7680 format=2)
[22104758486] [INFO] [flytrap] [CPU0] FLYTRAP: Created Limine ContentSource node
[22105816022] [INFO] [flytrap] [CPU0] FLYTRAP: Performing initial boot module scan...
[22138616183] [INFO] [bloom::compositor] [CPU0] bloom: display backend: BootFB
[22150717087] [INFO] [bloom::compositor] [CPU0] bloom: mapped size=8294400 (source=bytespace_info)
[22248916434] [INFO] [bloom::present] [CPU0] bloom: driver REGISTER (kind=1 caps=0x3)
[22252932852] [INFO] [bloom::present] [CPU0] display: negotiated proto 1.0 caps=DIRTY_RECTS|FULLFRAME max_rects=32
[22378416745] [INFO] [sprout::supervisor] [CPU0] SPROUT: Launching app '/boot/fontd'
[22380196881] [INFO] [kernel::task::loader] [CPU0] Loading module: /boot/fontd
[22380724780] [DEBUG] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[22381855923] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[22415925123] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=226000 exec=false
[22422546333] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=22d000 exec=false
[22431354648] [DEBUG] [kernel::task::scheduler::spawn] [CPU0] SCHED: Task 13 (user task/process) assigned to CPU 0
[22433588510] [INFO] [sprout::supervisor] [CPU0] SPROUT: App launched (PID=13)
[22434760939] [INFO] [sprout::supervisor] [CPU0] SPROUT: Launching app '/boot/blossom'
[22436013681] [INFO] [kernel::task::loader] [CPU0] Loading module: /boot/blossom
[22436499609] [DEBUG] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[22437512552] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[22459944404] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=218000 exec=false
[22462872078] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=21b000 exec=false
[22472191568] [DEBUG] [kernel::task::scheduler::spawn] [CPU0] SCHED: Task 14 (user task/process) assigned to CPU 0
[22474347987] [INFO] [sprout::supervisor] [CPU0] SPROUT: App launched (PID=14)
[22476011770] [INFO] [sprout::supervisor] [CPU0] SPROUT: Launching app '/boot/cambium'
[22478639288] [INFO] [sprout::supervisor] [CPU0] SPROUT: Failed to launch app '/boot/cambium': ENOENT
[22479645131] [INFO] [sprout::supervisor] [CPU0] SPROUT: Launching app '/boot/ahci_disk'
[22480927161] [INFO] [kernel::task::loader] [CPU0] Loading module: /boot/ahci_disk
[22481393870] [DEBUG] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 03, 00, 00, 00, 00, 00, 00, 00, 00]
[22482336614] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[22490095231] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=207000 exec=false
[22493030892] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=20a000 exec=false
[22502850671] [DEBUG] [kernel::task::scheduler::spawn] [CPU0] SCHED: Task 15 (user task/process) assigned to CPU 0
[22504992668] [INFO] [sprout::supervisor] [CPU0] SPROUT: App launched (PID=15)
[22506640030] [INFO] [sprout::supervisor] [CPU0] SPROUT: Launching app '/boot/iso_reader'
[22507604184] [INFO] [sprout::supervisor] [CPU0] SPROUT: Failed to launch app '/boot/iso_reader': ENOENT
[22508638625] [INFO] [sprout::supervisor] [CPU0] SPROUT: Launching app '/boot/font_explorer'
[22509567862] [INFO] [sprout::supervisor] [CPU0] SPROUT: Failed to launch app '/boot/font_explorer': ENOENT
[22510615598] [INFO] [sprout::supervisor] [CPU0] SPROUT: Launching app '/boot/photosynthesis'
[22512367175] [INFO] [kernel::task::loader] [CPU0] Loading module: /boot/photosynthesis
[22512988969] [DEBUG] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[22513970075] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[22559375882] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=233000 exec=false
[22567155114] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=23c000 exec=false
[22576851008] [DEBUG] [kernel::task::scheduler::spawn] [CPU0] SCHED: Task 16 (user task/process) assigned to CPU 0
[22579169254] [INFO] [sprout::supervisor] [CPU0] SPROUT: App launched (PID=16)
[22580444608] [INFO] [sprout::supervisor] [CPU0] SPROUT: Launching app '/boot/nectar'
[22581810654] [INFO] [kernel::task::loader] [CPU0] Loading module: /boot/nectar
[22582317430] [DEBUG] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[22583270661] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[22595879144] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=20c000 exec=false
[22598664106] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=20f000 exec=false
[22606885060] [DEBUG] [kernel::task::scheduler::spawn] [CPU0] SCHED: Task 17 (user task/process) assigned to CPU 0
[22609082625] [INFO] [sprout::supervisor] [CPU0] SPROUT: App launched (PID=17)
[22610679707] [INFO] [sprout::supervisor] [CPU0] SPROUT: Launching app '/boot/fetchd'
[22612113013] [INFO] [kernel::task::loader] [CPU0] Loading module: /boot/fetchd
[22612624305] [DEBUG] [kernel::task::loader] [CPU0]   Header: [7f, 45, 4c, 46, 02, 01, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00]
[22613713640] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=200000 exec=true
[22624964473] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=20b000 exec=false
[22627755082] [INFO] [kernel::task::loader] [CPU0] Segment: vaddr=20e000 exec=false
[22636002670] [DEBUG] [kernel::task::scheduler::spawn] [CPU0] SCHED: Task 18 (user task/process) assigned to CPU 0
[22638380074] [INFO] [sprout::supervisor] [CPU0] SPROUT: App launched (PID=18)
[22639715135] [INFO] [sprout::supervisor] [CPU0] SPROUT: Startup complete. Entering idle loop.
[22649745635] [DEBUG] [kernel::task::scheduler::spawn] [CPU0] Trampoline entered. Arg: 0xffffffffb0004608
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[22651189769] [INFO] [bran::arch::x86_64::enter_user] [CPU0] Entering user mode tid=13 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562158925 RSP_BEFORE=18446744072368807856 RFLAGS_BEFORE=130 CR3_BEFORE=69812224 fs_base=0 gs_base=18446744071564147976
[22655030894] [INFO] [fontd] [CPU0] FONTD: Starting Graph-Native Font Service v2 (Atlas-based IPC)
[22661823730] [DEBUG] [kernel::task::scheduler::spawn] [CPU0] Trampoline entered. Arg: 0xffffffffb0045d20
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[22663114499] [INFO] [bran::arch::x86_64::enter_user] [CPU0] Entering user mode tid=14 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562158925 RSP_BEFORE=18446744072368873392 RFLAGS_BEFORE=130 CR3_BEFORE=70086656 fs_base=0 gs_base=18446744071564147976
[22667580665] [INFO] [blossom] [CPU0] BLOSSOM: Starting SVG Cache Service
[22668935610] [INFO] [blossom] [CPU0] BLOSSOM: Init UI pipeline...
[22671023125] [DEBUG] [kernel::task::scheduler::spawn] [CPU0] Trampoline entered. Arg: 0xffffffffb0047ff0
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[22672213127] [INFO] [bran::arch::x86_64::enter_user] [CPU0] Entering user mode tid=15 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562158925 RSP_BEFORE=18446744072368938928 RFLAGS_BEFORE=130 CR3_BEFORE=70287360 fs_base=0 gs_base=18446744071564147976
[22781778554] [INFO] [ahci_disk] [CPU0] AHCI: Starting AHCI/SATA disk driver v1
[22785835156] [DEBUG] [kernel::task::scheduler::spawn] [CPU0] Trampoline entered. Arg: 0xffffffffb004a9e0
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[22787147546] [INFO] [bran::arch::x86_64::enter_user] [CPU0] Entering user mode tid=16 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562158925 RSP_BEFORE=18446744072369004464 RFLAGS_BEFORE=130 CR3_BEFORE=70426624 fs_base=0 gs_base=18446744071564147976
[22916599561] [INFO] [photosynthesis] [CPU0] Photosynthesis starting...
[22919513772] [DEBUG] [kernel::task::scheduler::spawn] [CPU0] Trampoline entered. Arg: 0xffffffffb004c158
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[22920776072] [INFO] [bran::arch::x86_64::enter_user] [CPU0] Entering user mode tid=17 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562158925 RSP_BEFORE=18446744072369070000 RFLAGS_BEFORE=130 CR3_BEFORE=70762496 fs_base=0 gs_base=18446744071564147976
[22924089842] [INFO] [nectar] [CPU0] NECTAR: Started.
[22926800921] [DEBUG] [kernel::task::scheduler::spawn] [CPU0] Trampoline entered. Arg: 0xffffffffb004c468
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
[22927974978] [INFO] [bran::arch::x86_64::enter_user] [CPU0] Entering user mode tid=18 target_pc=2097152 target_sp=8388608 target_cs=43 target_ss=35 CS=8 SS=16 CPL_KERNEL_BEFORE=0 RIP_BEFORE=18446744071562158925 RSP_BEFORE=18446744072369135536 RFLAGS_BEFORE=130 CR3_BEFORE=70914048 fs_base=0 gs_base=18446744071564147976
[22931587529] [INFO] [fetchd] [CPU0] FETCHD: Starting IP address display...
[22932597054] [INFO] [fetchd] [CPU0] FETCHD: Waiting for UI Root (Compositor)...
[22948622587] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: ptr=0x7fbfb0
[22949988900] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: validating range len=48
[22951089235] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: copyin success. mode=1 start_seq=0
[22952585619] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=202 subj_lo=0
[22964988478] [INFO] [ahci_disk] [CPU0] AHCI: Found 6 PCI functions
[22973849734] [INFO] [stem::ui] [CPU0] UiBuilder: created root 194
[22989793927] [DEBUG] [kernel::task::scheduler::spawn] [CPU0] SCHED: Task 19 (user thread) assigned to CPU 0
[22992414921] [INFO] [bloom] [CPU0] bloom: spawned asset watcher (tid=19)
[22993349181] [INFO] [bloom::frame_loop] [CPU0] bloom: running (fps_target=60)
[22994673392] [INFO] [bloom] [CPU0] [bloom] bristle_evt_handle = 10 (from bristle_evt=10)
[23030040051] [INFO] [flytrap] [CPU0] FLYTRAP: Published new asset '/boot/sprout' (raw, 90640 bytes, hash=e536630b94edbbce)
T:BA80 [23075214024] [DEBUG] [bloom::painter_resources] [CPU0] [bloom] asset_watcher_entry: spawning sub-loaders
[23083908297] [DEBUG] [kernel::task::scheduler::spawn] [CPU0] SCHED: Task 20 (user thread) assigned to CPU 0
[23092870077] [DEBUG] [kernel::task::scheduler::spawn] [CPU0] SCHED: Task 21 (user thread) assigned to CPU 0
[23102464476] [DEBUG] [kernel::task::scheduler::spawn] [CPU0] SCHED: Task 22 (user thread) assigned to CPU 0
[23112333399] [INFO] [fontd] [CPU0] FONTD: Service node created, req=13, resp=16
[23114857705] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: ptr=0x7fbfb0
[23115783435] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: validating range len=48
[23116481687] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: copyin success. mode=1 start_seq=0
[23117227766] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=210 subj_lo=0
T:BFC0 T:BE30 T:A7F0 [23136883956] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: ptr=0x7f9fb0
[23137590802] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: validating range len=48
[23138484411] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: copyin success. mode=0 start_seq=0
[23139272491] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=212 pred=0 subj_lo=0
[23167623517] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: ptr=0x7fbfb0
[23168412938] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: validating range len=48
[23169111795] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: copyin success. mode=1 start_seq=0
[23169857933] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=214 subj_lo=0
[23179806305] [INFO] [bloom] [CPU0] bloom: No VirtioGpu - using CPU composition mode
[23185671542] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: ptr=0x7f9fb0
[23186408185] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: validating range len=48
[23187177612] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: copyin success. mode=0 start_seq=0
[23187831144] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=217 pred=0 subj_lo=0
[23201080127] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: ptr=0x7f61a0
[23201806087] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: validating range len=48
[23202625143] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: copyin success. mode=1 start_seq=0
[23203492578] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=218 subj_lo=0
[23216794675] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: ptr=0x7fbfb0
[23217508609] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: validating range len=48
[23218201193] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: copyin success. mode=1 start_seq=0
[23219074696] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=219 subj_lo=0
[23232265998] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: ptr=0x7f9fb0
[23233045369] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: validating range len=48
[23233699863] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: copyin success. mode=0 start_seq=0
[23234339928] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=194 pred=0 subj_lo=0
[23239867429] [INFO] [photosynthesis] [CPU0] Found UI Root: 194
[23246317980] [INFO] [nectar] [CPU0] NECTAR: Found UI_CROWN: Some(ThingId([194, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]))
[23282443500] [INFO] [fetchd] [CPU0] FETCHD: Found UI Root: 194
[23293918945] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: ptr=0x7f61a0
[23294862770] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: validating range len=48
[23295544250] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: copyin success. mode=1 start_seq=0
[23296219743] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=220 pred=0 subj_lo=0
[23303723851] [INFO] [fontd] [CPU0] FONTD: Opened ASSET watch (handle=203) for kind 'Asset'
[23304829977] [INFO] [fontd] [CPU0] FONTD: Service ready
[23314003513] [INFO] [nectar] [CPU0] NECTAR: Created UI_WINDOW node: ThingId([205, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[23319689918] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: ptr=0x7fbfb0
[23320479583] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: validating range len=48
[23321163636] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: copyin success. mode=1 start_seq=0
[23322020462] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=221 subj_lo=0
[23366387977] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: ptr=0x7f61a0
[23367167095] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: validating range len=48
[23367947710] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: copyin success. mode=1 start_seq=0
[23369075509] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=225 subj_lo=0
[23380755859] [DEBUG] [bloom::painter_resources] [CPU0] [bloom] wallpaper loader: loading linen.bmp
[23390056100] [DEBUG] [kernel::task::scheduler::spawn] [CPU0] SCHED: Task 23 (user thread) assigned to CPU 0
[23391861842] [DEBUG] [bloom::asset] [CPU0] [asset_bank] worker spawned tid=23 (priority=2)
[23393503424] [DEBUG] [bloom::painter_resources] [CPU0] [bloom] cursor loader: loading default cursor
[23396559197] [DEBUG] [bloom::painter_resources] [CPU0] [bloom] icon loader started
[23405793500] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: ptr=0x7fbfb0
[23406491130] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: validating range len=48
[23407156295] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: copyin success. mode=1 start_seq=0
[23408037535] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: DECODED FILTER: flags=0x2 kind=0 pred=227 subj_lo=0
[23423283842] [INFO] [bloom] [CPU0] [bloom] Starting UI loop immediately (not waiting for fonts)
T:CA10 [23434866306] [DEBUG] [bloom::asset] [CPU0] [asset_bank] worker started (priority bump)
[23436240546] [DEBUG] [bloom::asset] [CPU0] [asset_bank] load_wallpaper_immediate: linen.bmp
[23475746738] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: ptr=0x7fbfb0
[23476617027] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: validating range len=48
[23477300362] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: copyin success. mode=1 start_seq=0
[23478165479] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=220 pred=0 subj_lo=0
[23490828170] [INFO] [flytrap] [CPU0] FLYTRAP: Created File node '/boot/sprout' (90640 bytes, hash=e536630b94edbbce)
[23529681540] [INFO] [nectar] [CPU0] NECTAR: Initial UI scene published.
[23531245799] [INFO] [nectar] [CPU0] NECTAR: Networking setup starting...
[23566767740] [INFO] [ahci_disk] [CPU0] AHCI: Found AHCI controller at PCI func ThingId([174, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
[23589470704] [INFO] [ahci_disk] [CPU0] AHCI: BAR5=0x810c4000
[23590330537] [INFO] [kernel::syscall::handlers::device] [CPU0] DEVICE: task 15 claimed device 174 (handle 1)
[23591501993] [INFO] [ahci_disk] [CPU0] AHCI: Claimed PCI device ThingId([174, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) handle=1
[23592986690] [INFO] [kernel::syscall::handlers::device] [CPU0] DEVICE: Mapped BAR5 phys=0x810c4000 size=0x1000 -> virt=0x117d4000
[23594234838] [INFO] [ahci_disk] [CPU0] AHCI: Mapped ABAR at 0x117d4000
[23596219876] [INFO] [ahci_disk] [CPU0] AHCI: Version 1.0, 6 ports, 32 slots, 64-bit: true
[23597379253] [INFO] [ahci_disk] [CPU0] AHCI: Ports implemented: 0x3f
[23598628732] [INFO] [ahci_disk] [CPU0] AHCI: DMA virt=0x20b000
[23601925928] [INFO] [ahci_disk] [CPU0] AHCI: DMA phys=0x4317000
[23602974085] [INFO] [ahci_disk] [CPU0] AHCI: Probing port 0...
[23603886885] [INFO] [ahci_disk] [CPU0] AHCI: Port 0 - no device
[23604685370] [INFO] [ahci_disk] [CPU0] AHCI: Probing port 1...
[23605509343] [INFO] [ahci_disk] [CPU0] AHCI: Port 1 - no device
[23606247000] [INFO] [ahci_disk] [CPU0] AHCI: Probing port 2...
[23607342043] [INFO] [ahci_disk] [CPU0] AHCI: Port 2 - SATAPI drive (sig=0xeb140101)
[23653372218] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: ptr=0x7ffe90
[23654186183] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: validating range len=48
[23654881523] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: copyin success. mode=1 start_seq=0
[23655628439] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: DECODED FILTER: flags=0x0 kind=0 pred=0 subj_lo=0
[23842939807] [INFO] [blossom] [CPU0] BLOSSOM: UI pipeline ready
[23878539106] [INFO] [ahci_disk] [CPU0] AHCI: Registered ATAPI block device 219 port=2 model='                                        ' rpc_port=17
[23911408025] [INFO] [ahci_disk] [CPU0] AHCI: Probing port 3...
[23912249580] [INFO] [ahci_disk] [CPU0] AHCI: Port 3 - no device
[23912995362] [INFO] [ahci_disk] [CPU0] AHCI: Probing port 4...
[23913765182] [INFO] [ahci_disk] [CPU0] AHCI: Port 4 - no device
[23914462047] [INFO] [ahci_disk] [CPU0] AHCI: Probing port 5...
[23915166753] [INFO] [ahci_disk] [CPU0] AHCI: Port 5 - no device
[23916101873] [INFO] [ahci_disk] [CPU0] AHCI: Found 1 SATA disk(s)
[23916927106] [INFO] [ahci_disk] [CPU0] AHCI: Entering RPC service loop
[23961831003] [INFO] [blossom] [CPU0] BLOSSOM: Service node created, req=19, resp=22
[23962824027] [INFO] [blossom] [CPU0] BLOSSOM: Service ready
[23984222369] [INFO] [fetchd] [CPU0] FETCHD: Entering main loop, watching for network stack...
[24109238251] [INFO] [flytrap] [CPU0] FLYTRAP: Published new asset '/boot/bristle' (raw, 25008 bytes, hash=b720f78090e9469f)
[24301542405] [DEBUG] [bloom::asset] [CPU0] [asset_bank] mapping bytespace 143 (4718646 bytes) for 'linen.bmp'
[24342567269] [DEBUG] [bloom::asset] [CPU0] [asset_bank] decoding BMP for 'linen.bmp'...
[25385470072] [DEBUG] [bloom::asset] [CPU0] [asset_bank] BMP decoded: 1536x1024 for 'linen.bmp'
[26452687725] [INFO] [bloom::present] [CPU0] display: full-frame damage, using full-frame present
[26464251446] [DEBUG] [bloom::asset] [CPU0] [asset_bank] publish_wallpaper (pending): 1536x1024
[26466533390] [DEBUG] [bloom::asset] [CPU0] [asset_bank] load_cursor_immediate: /assets/cursors/future/default.svg
[26505142841] [INFO] [flytrap] [CPU0] FLYTRAP: Created File node '/boot/bristle' (25008 bytes, hash=b720f78090e9469f)
[26540308264] [INFO] [bloom] [CPU0] [CONTRACT] [bloom] First frame rendered
[26562448368] [DEBUG] [bloom::reclaimer] [CPU0] [reclaimer] +6291456 bytes (total: 6291456)
[26563675456] [INFO] [bloom::asset] [CPU0] [asset_bank] promoting wallpaper 'linen.bmp' to gen=1 (6291456b)
[26636635016] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: ptr=0x400000040870
[26637406492] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: validating range len=48
[26638067876] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: copyin success. mode=1 start_seq=0
[26638874679] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=252 pred=0 subj_lo=0
[26691932332] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: ptr=0x400000040870
[26692704318] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: validating range len=48
[26693359226] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: copyin success. mode=1 start_seq=0
[26694184080] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=254 pred=0 subj_lo=0
[26949784357] [INFO] [flytrap] [CPU0] FLYTRAP: Published new asset '/boot/rtc_cmos' (raw, 33368 bytes, hash=d39bfb261fe31500)
[27009957113] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: ptr=0x400000040870
[27010894206] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: validating range len=48
[27011691904] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: copyin success. mode=1 start_seq=0
[27012495277] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=255 pred=0 subj_lo=0
[27113903531] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: ptr=0x400000040870
[27114719408] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: validating range len=48
[27115377481] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: copyin success. mode=1 start_seq=0
[27116068480] [INFO] [kernel::syscall::handlers::root_handlers] [CPU0] sys_root_watch_open: DECODED FILTER: flags=0x1 kind=256 pred=0 subj_lo=0
[27241747701] [DEBUG] [bloom::asset] [CPU0] [asset_bank] mapping bytespace 152 (3051 bytes)
[27257841262] [INFO] [flytrap] [CPU0] FLYTRAP: Created File node '/boot/rtc_cmos' (33368 bytes, hash=d39bfb261fe31500)
[27262532672] [DEBUG] [bloom::asset] [CPU0] [asset_bank] mapped to 0x12c38000
[27263579417] [DEBUG] [bloom::asset] [CPU0] [asset_bank] detected SVG format
[27264684008] [DEBUG] [bloom::asset] [CPU0] [asset_bank] parsing SVG cursor from /assets/cursors/future/default.svg
[27373131731] [INFO] [bloom::raster] [CPU0] [raster] build_edges: verbs=27 flat_sq=0.003125 edges=36
[27386355572] [INFO] [bloom::raster] [CPU0] [raster] build_edges: verbs=10 flat_sq=0.003125 edges=19
[27391971702] [INFO] [bloom::raster] [CPU0] [raster] build_edges: verbs=60 flat_sq=0.003125 edges=45
[27394002189] [INFO] [bloom::raster] [CPU0] [raster] build_edges: verbs=10 flat_sq=0.003125 edges=32
[27396061136] [INFO] [bloom::raster] [CPU0] [raster] build_edges: verbs=95 flat_sq=0.003125 edges=72
[27401391197] [DEBUG] [bloom::asset] [CPU0] [asset_bank] SUCCESS: SVG cursor rasterized 64x64
[27424604590] [DEBUG] [bloom::asset] [CPU0] [asset_bank] publish_cursor (pending)
[27425726934] [DEBUG] [bloom::asset] [CPU0] [asset_bank] cursor: Static frame 64x64 hotspot (12, 8)
[27427177919] [DEBUG] [bloom::asset] [CPU0] [asset_bank] mapping font bytespace 146 (569208 bytes)
[27447998398] [DEBUG] [bloom::asset] [CPU0] [asset_bank] mapped at 0x12c39000
[27449093933] [DEBUG] [bloom::asset] [CPU0] [asset_bank] parsing font '/assets/fonts/NotoSans-Regular.ttf'...

```
</details>
