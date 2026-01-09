# ✅ Scenario: Heap Growth on Demand

> Last run: 2026-01-09 13:03:50

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booting | ✅ | 347ms | <a href="./01/after.png"><img src="./01/after.png" width="150" /></a> - [💾](./01/registers.txt) |
| 2 | Then the log should contain "kheap: forcing growth..." | ✅ | 4507ms | <a href="./02/after.png"><img src="./02/after.png" width="150" /></a> - [💾](./02/registers.txt) |
| 3 | And the log should contain "kheap: grew by 16 pages" | ✅ | 724ms | <a href="./03/after.png"><img src="./03/after.png" width="150" /></a> [📜](./03/serial.log) [💾](./03/registers.txt) |
| 4 | And the log should contain "kheap: big allocation ok" | ✅ | 217ms | - [📜](./04/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[12218622933] [INFO] System booted
[12231476796] [INFO] boot: phys ranges=35 modules=0
[12236307072] [INFO] BootHeap initialized. Range: 0xffffff8040000000 - 0xffffff8041000000
[12241174671] [INFO] Initializing Real Frame Allocator...
[12243098637] [INFO] frame_alloc: base=0x0 frames=516638 words=8073
[12246420978] [INFO] Allocating bitmap of 8073 words...
[12253560990] [INFO] Bitmap allocated at 0xffffff8040000000
[12258751758] [INFO] Zeroing bitmap...
[12271599285] [INFO] Bitmap zeroed.
[12766791444] [INFO] frame_alloc: total=516638 free=511759 used=4879
[12771042735] [INFO] Running frame_alloc sanity check...
[12774795462] [INFO] frame_alloc: sanity: single ok
[12781706025] [INFO] frame_alloc: sanity: contig(8) ok
[12784126773] [INFO] Testing paging subsystem...
[12793153725] [INFO] Switched to new address space
[12796119105] [INFO] Paging subsystem test passed
[12796579125] [INFO] Paging subsystem test passed
[12797009214] [INFO] Initializing Kernel Heap...
[12804197406] [INFO] kheap: grew by 64 pages (phys=0x4f000, virt=0xffffa00000000000)
[12809570169] [INFO] global_alloc: switched to kernel heap
[12810510306] [INFO] Running heap sanity check...
[12831064884] [INFO] kheap: sanity ok
[12834228165] [INFO] kheap: forcing growth...
[12838145991] [INFO] kheap: grew by 16 pages (phys=0x100000, virt=0xffffa00000040000)
[12865065741] [INFO] kheap: big allocation ok (len=307200)
[12868917171] [INFO] kheap: reserved=268435456 committed=327680
[12878853141] [INFO] Initializing Task System...
[12887105286] [INFO] threads: supported
[12887525211] [INFO] Spawning Thread A...
[12895738515] [INFO] Spawning Thread B...
[12902560638] [INFO] Entering Scheduler Loop (Main Task)...
[12909508755] [INFO] Thread A (arg=1) ticks=12909332469
[13339796316] [INFO] Thread B (arg=2) ticks=13339594686
[13804128954] [INFO] Thread A (arg=1) ticks=13804096284
[14266819347] [INFO] Thread B (arg=2) ticks=14266783410
[14730984906] [INFO] Thread A (arg=1) ticks=14730954711
[15248252547] [INFO] Thread B (arg=2) ticks=15248220240
[16083326292] [INFO] Thread A (arg=1) ticks=16083291972
[16834172091] [INFO] Thread B (arg=2) ticks=16834137474
[17277654273] [INFO] Thread A (arg=1) ticks=17277617082
[18183784707] [INFO] Thread B (arg=2) ticks=18183750057
[18853948212] [INFO] Thread A (arg=1) ticks=18853913397
[19447352298] [INFO] Thread B (arg=2) ticks=19447317846
[19715780271] [INFO] Thread A (arg=1) ticks=19715749053
[19968209811] [INFO] Thread B (arg=2) ticks=19968180078
[20215314141] [INFO] Thread A (arg=1) ticks=20215282098
[20459048940] [INFO] Thread B (arg=2) ticks=20459021451
[20739371268] [INFO] Thread A (arg=1) ticks=20739340743
[20983517841] [INFO] Thread B (arg=2) ticks=20983496556
[21227005437] [INFO] Thread A (arg=1) ticks=21226978608
[21468462807] [INFO] Thread B (arg=2) ticks=21468437166
[21711288621] [INFO] Thread A (arg=1) ticks=21711256314
[21954885381] [INFO] Thread B (arg=2) ticks=21954869244
[22198891869] [INFO] Thread A (arg=1) ticks=22198869000
[22448939271] [INFO] Thread B (arg=2) ticks=22448920923
[22692484155] [INFO] Thread A (arg=1) ticks=22692465807
[22936651782] [INFO] Thread B (arg=2) ticks=22936628451
[23178976458] [INFO] Thread A (arg=1) ticks=23178944547
[23430399432] [INFO] Thread B (arg=2) ticks=23430246543
[23693893575] [INFO] Thread A (arg=1) ticks=23693857737
[23964196674] [INFO] Thread B (arg=2) ticks=23964163344
[24222527076] [INFO] Thread A (arg=1) ticks=24222495099
[24494924091] [INFO] Thread B (arg=2) ticks=24494890497
[24767286027] [INFO] Thread A (arg=1) ticks=24767259198
[25031441523] [INFO] Thread B (arg=2) ticks=25031405586
[25457008797] [INFO] Thread A (arg=1) ticks=25456961805
[25743203079] [INFO] Thread B (arg=2) ticks=25743173049
[26026612953] [INFO] Thread A (arg=1) ticks=26026581372
[26490708123] [INFO] Thread B (arg=2) ticks=26490673077
[26987841243] [INFO] Thread A (arg=1) ticks=26987809035
[27300068763] [INFO] Thread B (arg=2) ticks=27300035862
[27798054405] [INFO] Thread A (arg=1) ticks=27798023847
[28075851639] [INFO] Thread B (arg=2) ticks=28075819926
[28336076109] [INFO] Thread A (arg=1) ticks=28336044528

```
</details>
