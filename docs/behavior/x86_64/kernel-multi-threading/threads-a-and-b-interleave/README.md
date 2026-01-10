# ❌ Scenario: Threads A and B interleave

> Last run: 2026-01-10 11:17:34

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 4970ms | <a href="./01/after.png"><img src="./01/after.png" width="150" /></a> [📜](./01/serial.log) [💾](./01/registers.txt) |
| 2 | Then I should see "Spawning Thread A..." | ✅ | 757ms | <a href="./02/after.png"><img src="./02/after.png" width="150" /></a> [📜](./02/serial.log) [💾](./02/registers.txt) |
| 3 | And I should see "Spawning Thread B..." | ✅ | 626ms | <a href="./03/after.png"><img src="./03/after.png" width="150" /></a> [📜](./03/serial.log) [💾](./03/registers.txt) |
| 4 | And I should see "Thread A (arg=1) ticks=" | ✅ | 717ms | <a href="./04/after.png"><img src="./04/after.png" width="150" /></a> [📜](./04/serial.log) [💾](./04/registers.txt) |
| 5 | And "Thread A (arg=1) ticks=" should appear at least 2 times | ✅ | 622ms | <a href="./05/after.png"><img src="./05/after.png" width="150" /></a> [📜](./05/serial.log) [💾](./05/registers.txt) |
| 6 | And I should see "Thread B (arg=2) ticks=" | ✅ | 757ms | <a href="./06/after.png"><img src="./06/after.png" width="150" /></a> [📜](./06/serial.log) [💾](./06/registers.txt) |
| 7 | And "Thread B (arg=2) ticks=" should appear at least 2 times | ✅ | 781ms | <a href="./07/after.png"><img src="./07/after.png" width="150" /></a> [📜](./07/serial.log) [💾](./07/registers.txt) |
| 8 | And I should see "Thread A (arg=1) ticks=" after "Thread B (arg=2) ticks=" | ❌ | 13ms | - [📜](./08/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[13573239240] [INFO] thing-os kernel v0.1.0 starting...
[13584674367] [INFO] Intent-Mechanism paging split active
[13585698027] [INFO] System booted
[13590285687] [INFO] Memory map has 35 entries
[13595782761] [INFO]   [0] 0x0 - 0xa0000 (Usable)
[13597893210] [INFO]   [1] 0x100000 - 0x800000 (Usable)
[13598311188] [INFO]   [2] 0x800000 - 0x808000 (Other)
[13598762562] [INFO]   [3] 0x808000 - 0x80b000 (Usable)
[13599178098] [INFO]   [4] 0x80b000 - 0x80c000 (Other)
[13599612576] [INFO]   [5] 0x80c000 - 0x811000 (Usable)
[13600031544] [INFO]   [6] 0x811000 - 0x900000 (Other)
[13600445298] [INFO]   [7] 0x900000 - 0x1780000 (Reserved)
[13600913403] [INFO]   [8] 0x1780000 - 0x79fa6000 (Usable)
[13601352006] [INFO]   [9] 0x79fa6000 - 0x7a16c000 (Reserved)
[13602095793] [INFO] HHDM Offset: 0xffff800000000000
[14187014886] [INFO] Frame allocator initialized with 511944 free frames
[14243911935] [INFO] Spawning Thread A...
[14264920098] [INFO] Spawning Thread B...
[14280939618] [INFO] System initialized. Entering scheduler loop.
[14296215945] [INFO] Thread A (arg=1) ticks=14295986661
[14970883752] [INFO] Thread B (arg=2) ticks=14970689448
[15483117705] [INFO] Thread A (arg=1) ticks=15483091239
[15990883557] [INFO] Thread B (arg=2) ticks=15990852339
[16547966226] [INFO] Thread A (arg=1) ticks=16547937648
[17059837377] [INFO] Thread B (arg=2) ticks=17059807644
[17826410877] [INFO] Thread A (arg=1) ticks=17826383355
[18329736183] [INFO] Thread B (arg=2) ticks=18329718891
[18867747921] [INFO] Thread A (arg=1) ticks=18867721389
[19301233842] [INFO] Thread B (arg=2) ticks=19301203812
[19734491040] [INFO] Thread A (arg=1) ticks=19734465861
[20247062517] [INFO] Thread B (arg=2) ticks=20247035886
[20750337036] [INFO] Thread A (arg=1) ticks=20750308557
[21188542716] [INFO] Thread B (arg=2) ticks=21188514468
[21623888121] [INFO] Thread A (arg=1) ticks=21623861424
[22063888638] [INFO] Thread B (arg=2) ticks=22063861941
[22499304003] [INFO] Thread A (arg=1) ticks=22499278791
[22941167568] [INFO] Thread B (arg=2) ticks=22941137670
[23375391468] [INFO] Thread A (arg=1) ticks=23375366157
[23848746603] [INFO] Thread B (arg=2) ticks=23848717662
[24280963806] [INFO] Thread A (arg=1) ticks=24280937142
[24723045105] [INFO] Thread B (arg=2) ticks=24723014415
[25174026108] [INFO] Thread A (arg=1) ticks=25174000665
[25677475098] [INFO] Thread B (arg=2) ticks=25677444870
[26137953699] [INFO] Thread A (arg=1) ticks=26137927002
[26660809395] [INFO] Thread B (arg=2) ticks=26660781246
[27122458836] [INFO] Thread A (arg=1) ticks=27122432865
[27572648697] [INFO] Thread B (arg=2) ticks=27572621604
[28029687840] [INFO] Thread A (arg=1) ticks=28029661143
[28471386768] [INFO] Thread B (arg=2) ticks=28471360500
[28907046102] [INFO] Thread A (arg=1) ticks=28907019075
[29350006686] [INFO] Thread B (arg=2) ticks=29349977943
[29784858246] [INFO] Thread A (arg=1) ticks=29784832506
[30229661352] [INFO] Thread B (arg=2) ticks=30229633764
[30691405668] [INFO] Thread A (arg=1) ticks=30691378839
[31137496533] [INFO] Thread B (arg=2) ticks=31137469209
[31573355352] [INFO] Thread A (arg=1) ticks=31573327170
[32016432690] [INFO] Thread B (arg=2) ticks=32016405564
[32453021403] [INFO] Thread A (arg=1) ticks=32452980351
[32963898990] [INFO] Thread B (arg=2) ticks=32963871732
[33408741630] [INFO] Thread A (arg=1) ticks=33408715197
[33856911561] [INFO] Thread B (arg=2) ticks=33856883379
[34296366093] [INFO] Thread A (arg=1) ticks=34296338967
[34756024941] [INFO] Thread B (arg=2) ticks=34755997188
[35199229197] [INFO] Thread A (arg=1) ticks=35199202302
[35648282505] [INFO] Thread B (arg=2) ticks=35648255544
[36095291133] [INFO] Thread A (arg=1) ticks=36095264667
[36545676750] [INFO] Thread B (arg=2) ticks=36545647512
[37000075728] [INFO] Thread A (arg=1) ticks=37000047843
[37641806235] [INFO] Thread B (arg=2) ticks=37641778416
[38464761225] [INFO] Thread A (arg=1) ticks=38464736838
[39032614500] [INFO] Thread B (arg=2) ticks=39032585856
[39527684295] [INFO] Thread A (arg=1) ticks=39527653572
[39988828605] [INFO] Thread B (arg=2) ticks=39988801083
[40430218686] [INFO] Thread A (arg=1) ticks=40430190735
[40877288397] [INFO] Thread B (arg=2) ticks=40877259588
[41381129955] [INFO] Thread A (arg=1) ticks=41381103060
[41861476866] [INFO] Thread B (arg=2) ticks=41861446704
[42305373891] [INFO] Thread A (arg=1) ticks=42305346402
[42751471818] [INFO] Thread B (arg=2) ticks=42751444956
[43197432300] [INFO] Thread A (arg=1) ticks=43197403689
[43712519235] [INFO] Thread B (arg=2) ticks=43712490492
[44181994065] [INFO] Thread A (arg=1) ticks=44181966642
[44632239729] [INFO] Thread B (arg=2) ticks=44632209765
[45125452713] [INFO] Thread A (arg=1) ticks=45125426049
[45631430823] [INFO] Thread B (arg=2) ticks=45631402179
[46069000527] [INFO] Thread A (arg=1) ticks=46068973368
[46516326153] [INFO] Thread B (arg=2) ticks=46516296981

```
</details>
