# ✅ Scenario: Heap Initialization and Basic Allocation

> Last run: 2026-01-09 13:03:50

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booting | ✅ | 380ms | <a href="./01/after.png"><img src="./01/after.png" width="150" /></a> - [💾](./01/registers.txt) |
| 2 | Then the log should contain "Initializing Kernel Heap..." | ✅ | 4715ms | <a href="./02/after.png"><img src="./02/after.png" width="150" /></a> [📜](./02/serial.log) [💾](./02/registers.txt) |
| 3 | And the log should contain "kheap: grew by 64 pages" | ✅ | 1098ms | <a href="./03/after.png"><img src="./03/after.png" width="150" /></a> [📜](./03/serial.log) [💾](./03/registers.txt) |
| 4 | And the log should contain "global_alloc: switched to kernel heap" | ✅ | 1672ms | <a href="./04/after.png"><img src="./04/after.png" width="150" /></a> [📜](./04/serial.log) [💾](./04/registers.txt) |
| 5 | And the log should contain "kheap: sanity ok" | ✅ | 224ms | - [📜](./05/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[12862540284] [INFO] System booted
[12875016000] [INFO] boot: phys ranges=35 modules=0
[12881302170] [INFO] BootHeap initialized. Range: 0xffffff8040000000 - 0xffffff8041000000
[12884089152] [INFO] Initializing Real Frame Allocator...
[12888741228] [INFO] frame_alloc: base=0x0 frames=516638 words=8073
[12889316418] [INFO] Allocating bitmap of 8073 words...
[12895770954] [INFO] Bitmap allocated at 0xffffff8040000000
[12901182723] [INFO] Zeroing bitmap...
[12906925152] [INFO] Bitmap zeroed.
[13358557674] [INFO] frame_alloc: total=516638 free=511759 used=4879
[13360212987] [INFO] Running frame_alloc sanity check...
[13363623339] [INFO] frame_alloc: sanity: single ok
[13370423385] [INFO] frame_alloc: sanity: contig(8) ok
[13372723419] [INFO] Testing paging subsystem...
[13381283388] [INFO] Switched to new address space
[13385020011] [INFO] Paging subsystem test passed
[13385438748] [INFO] Paging subsystem test passed
[13385826201] [INFO] Initializing Kernel Heap...
[13393138869] [INFO] kheap: grew by 64 pages (phys=0x4f000, virt=0xffffa00000000000)
[13396434348] [INFO] global_alloc: switched to kernel heap
[13396922550] [INFO] Running heap sanity check...
[13416346944] [INFO] kheap: sanity ok
[13419652059] [INFO] kheap: forcing growth...
[13423517250] [INFO] kheap: grew by 16 pages (phys=0x100000, virt=0xffffa00000040000)
[13450415649] [INFO] kheap: big allocation ok (len=307200)
[13454889327] [INFO] kheap: reserved=268435456 committed=327680
[13465092894] [INFO] Initializing Task System...
[13472854824] [INFO] threads: supported
[13473492549] [INFO] Spawning Thread A...
[13482287808] [INFO] Spawning Thread B...
[13488363339] [INFO] Entering Scheduler Loop (Main Task)...
[13494966672] [INFO] Thread A (arg=1) ticks=13494807843
[13759448208] [INFO] Thread B (arg=2) ticks=13759261428
[14220430785] [INFO] Thread A (arg=1) ticks=14220399006
[14681092602] [INFO] Thread B (arg=2) ticks=14681058282
[15166534185] [INFO] Thread A (arg=1) ticks=15166503363
[15780855123] [INFO] Thread B (arg=2) ticks=15780822321
[16277293494] [INFO] Thread A (arg=1) ticks=16277262408
[16745483799] [INFO] Thread B (arg=2) ticks=16745453274
[17247702054] [INFO] Thread A (arg=1) ticks=17247668427
[17693846379] [INFO] Thread B (arg=2) ticks=17693815458
[18012014889] [INFO] Thread A (arg=1) ticks=18011985585
[18259355202] [INFO] Thread B (arg=2) ticks=18259323786
[18656638143] [INFO] Thread A (arg=1) ticks=18656606628
[18951220497] [INFO] Thread B (arg=2) ticks=18951186342
[19430336904] [INFO] Thread A (arg=1) ticks=19430188536
[19898577468] [INFO] Thread B (arg=2) ticks=19898546349
[20380068159] [INFO] Thread A (arg=1) ticks=20380033938
[20858194269] [INFO] Thread B (arg=2) ticks=20858162028
[21307094556] [INFO] Thread A (arg=1) ticks=21307059510
[21773014032] [INFO] Thread B (arg=2) ticks=21772981164
[22252012893] [INFO] Thread A (arg=1) ticks=22251976296
[22648489611] [INFO] Thread B (arg=2) ticks=22648455852
[23096660664] [INFO] Thread A (arg=1) ticks=23096621493
[23344236432] [INFO] Thread B (arg=2) ticks=23344203894
[23590845795] [INFO] Thread A (arg=1) ticks=23590810353
[23848607607] [INFO] Thread B (arg=2) ticks=23848575201
[24105153567] [INFO] Thread A (arg=1) ticks=24105121722
[24363738003] [INFO] Thread B (arg=2) ticks=24363704277
[24604278732] [INFO] Thread A (arg=1) ticks=24604243389
[24858159579] [INFO] Thread B (arg=2) ticks=24858126381
[25121661477] [INFO] Thread A (arg=1) ticks=25121628807
[25701453294] [INFO] Thread B (arg=2) ticks=25701420162
[26616244446] [INFO] Thread A (arg=1) ticks=26616209631
[27115758483] [INFO] Thread B (arg=2) ticks=27115723140
[27358568127] [INFO] Thread A (arg=1) ticks=27358530342
[27617281230] [INFO] Thread B (arg=2) ticks=27617247966
[27890981910] [INFO] Thread A (arg=1) ticks=27890948151
[28138211046] [INFO] Thread B (arg=2) ticks=28138180158
[28386770346] [INFO] Thread A (arg=1) ticks=28386737841
[28637987565] [INFO] Thread B (arg=2) ticks=28637956182
[29107660956] [INFO] Thread A (arg=1) ticks=29107627098
[29355884679] [INFO] Thread B (arg=2) ticks=29355857289
[29617244580] [INFO] Thread A (arg=1) ticks=29617212405
[29868114210] [INFO] Thread B (arg=2) ticks=29868082167
[30116256060] [INFO] Thread A (arg=1) ticks=30116221443
[30366597591] [INFO] Thread B (arg=2) ticks=30366563799
[30826531560] [INFO] Thread A (arg=1) ticks=30826500573
[31607732574] [INFO] Thread B (arg=2) ticks=31607698155
[32081835852] [INFO] Thread A (arg=1) ticks=32081801136
[32483680779] [INFO] Thread B (arg=2) ticks=32483647515
[32939697054] [INFO] Thread A (arg=1) ticks=32939665143
[33403768464] [INFO] Thread B (arg=2) ticks=33403734705
[33981071454] [INFO] Thread A (arg=1) ticks=33981032316
[34457517930] [INFO] Thread B (arg=2) ticks=34457485557
[34905397824] [INFO] Thread A (arg=1) ticks=34905363240
[35386065759] [INFO] Thread B (arg=2) ticks=35386030647
[35854109247] [INFO] Thread A (arg=1) ticks=35854073442

```
</details>
