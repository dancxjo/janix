# ❌ Scenario: Threads A and B interleave

> Last run: 2026-01-10 11:23:49

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 5185ms | <a href="./01/after.png"><img src="./01/after.png" width="150" /></a> [📜](./01/serial.log) [💾](./01/registers.txt) |
| 2 | Then I should see "Spawning Thread A..." | ✅ | 798ms | <a href="./02/after.png"><img src="./02/after.png" width="150" /></a> [📜](./02/serial.log) [💾](./02/registers.txt) |
| 3 | And I should see "Spawning Thread B..." | ✅ | 735ms | <a href="./03/after.png"><img src="./03/after.png" width="150" /></a> [📜](./03/serial.log) [💾](./03/registers.txt) |
| 4 | And I should see "Thread A (arg=1) ticks=" | ✅ | 616ms | <a href="./04/after.png"><img src="./04/after.png" width="150" /></a> [📜](./04/serial.log) [💾](./04/registers.txt) |
| 5 | And "Thread A (arg=1) ticks=" should appear at least 2 times | ✅ | 689ms | <a href="./05/after.png"><img src="./05/after.png" width="150" /></a> [📜](./05/serial.log) [💾](./05/registers.txt) |
| 6 | And I should see "Thread B (arg=2) ticks=" | ✅ | 828ms | <a href="./06/after.png"><img src="./06/after.png" width="150" /></a> [📜](./06/serial.log) [💾](./06/registers.txt) |
| 7 | And "Thread B (arg=2) ticks=" should appear at least 2 times | ✅ | 713ms | <a href="./07/after.png"><img src="./07/after.png" width="150" /></a> [📜](./07/serial.log) [💾](./07/registers.txt) |
| 8 | And I should see "Thread A (arg=1) ticks=" after "Thread B (arg=2) ticks=" | ❌ | 9ms | - [📜](./08/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[13895474868] [INFO] thing-os kernel v0.1.0 starting...
[13906482612] [INFO] Intent-Mechanism paging split active
[13911151749] [INFO] System booted
[13918210812] [INFO] Memory map has 35 entries
[13921566285] [INFO]   [0] 0x0 - 0xa0000 (Usable)
[13922421051] [INFO]   [1] 0x100000 - 0x800000 (Usable)
[13922837346] [INFO]   [2] 0x800000 - 0x808000 (Other)
[13923272880] [INFO]   [3] 0x808000 - 0x80b000 (Usable)
[13923663963] [INFO]   [4] 0x80b000 - 0x80c000 (Other)
[13924047555] [INFO]   [5] 0x80c000 - 0x811000 (Usable)
[13924437615] [INFO]   [6] 0x811000 - 0x900000 (Other)
[13924821141] [INFO]   [7] 0x900000 - 0x1780000 (Reserved)
[13925254926] [INFO]   [8] 0x1780000 - 0x79fa6000 (Usable)
[13925666436] [INFO]   [9] 0x79fa6000 - 0x7a16c000 (Reserved)
[13926402270] [INFO] HHDM Offset: 0xffff800000000000
[14621869911] [INFO] Frame allocator initialized with 511944 free frames
[14656644618] [INFO] Spawning Thread A...
[14668438158] [INFO] Spawning Thread B...
[14677125243] [INFO] System initialized. Entering scheduler loop.
[14688317952] [INFO] Thread A (arg=1) ticks=14688077778
[15369525204] [INFO] Thread B (arg=2) ticks=15369331725
[16304305512] [INFO] Thread A (arg=1) ticks=16304279541
[16765266969] [INFO] Thread B (arg=2) ticks=16765239282
[17289595785] [INFO] Thread A (arg=1) ticks=17289570045
[18185579148] [INFO] Thread B (arg=2) ticks=18185551824
[18681163017] [INFO] Thread A (arg=1) ticks=18681136188
[19326833592] [INFO] Thread B (arg=2) ticks=19326805476
[19907732922] [INFO] Thread A (arg=1) ticks=19907705301
[20355271266] [INFO] Thread B (arg=2) ticks=20355242358
[20782484019] [INFO] Thread A (arg=1) ticks=20782458147
[21298341273] [INFO] Thread B (arg=2) ticks=21298317051
[22250194230] [INFO] Thread A (arg=1) ticks=22250168787
[22753591971] [INFO] Thread B (arg=2) ticks=22753560555
[23211589698] [INFO] Thread A (arg=1) ticks=23211562605
[23692800747] [INFO] Thread B (arg=2) ticks=23692769232
[24194568288] [INFO] Thread A (arg=1) ticks=24194542416
[24719667357] [INFO] Thread B (arg=2) ticks=24719638020
[25179153549] [INFO] Thread A (arg=1) ticks=25179128634
[25631915232] [INFO] Thread B (arg=2) ticks=25631885829
[26077683654] [INFO] Thread A (arg=1) ticks=26077656000
[26530682673] [INFO] Thread B (arg=2) ticks=26530653072
[26991317067] [INFO] Thread A (arg=1) ticks=26991288852
[27576377169] [INFO] Thread B (arg=2) ticks=27576349053
[28054561458] [INFO] Thread A (arg=1) ticks=28054533870
[28501308726] [INFO] Thread B (arg=2) ticks=28501282260
[29189409678] [INFO] Thread A (arg=1) ticks=29189370342
[29672964981] [INFO] Thread B (arg=2) ticks=29672934918
[30133598352] [INFO] Thread A (arg=1) ticks=30133569378
[30771366846] [INFO] Thread B (arg=2) ticks=30771339390
[31325647089] [INFO] Thread A (arg=1) ticks=31325620854
[31827609957] [INFO] Thread B (arg=2) ticks=31827580158
[32404646901] [INFO] Thread A (arg=1) ticks=32404618983
[32880467499] [INFO] Thread B (arg=2) ticks=32880441132
[33355182399] [INFO] Thread A (arg=1) ticks=33355156197
[33864270561] [INFO] Thread B (arg=2) ticks=33864242742
[34350403989] [INFO] Thread A (arg=1) ticks=34350378150
[34825457799] [INFO] Thread B (arg=2) ticks=34825427967
[35382987981] [INFO] Thread A (arg=1) ticks=35382959997
[35899435242] [INFO] Thread B (arg=2) ticks=35899408116
[36396077949] [INFO] Thread A (arg=1) ticks=36396052770
[36990549816] [INFO] Thread B (arg=2) ticks=36990523086
[37662112554] [INFO] Thread A (arg=1) ticks=37662084141
[38279873940] [INFO] Thread B (arg=2) ticks=38279845989
[38757625731] [INFO] Thread A (arg=1) ticks=38757600222
[39211326594] [INFO] Thread B (arg=2) ticks=39211298280
[39634241427] [INFO] Thread A (arg=1) ticks=39634211925
[40066804371] [INFO] Thread B (arg=2) ticks=40066774077
[40491845361] [INFO] Thread A (arg=1) ticks=40491817872
[40951907733] [INFO] Thread B (arg=2) ticks=40951878858
[41374636611] [INFO] Thread A (arg=1) ticks=41374609419
[42076045110] [INFO] Thread B (arg=2) ticks=42076014420
[42540091902] [INFO] Thread A (arg=1) ticks=42540065766
[42969450612] [INFO] Thread B (arg=2) ticks=42969421638
[43394164077] [INFO] Thread A (arg=1) ticks=43394137050
[43821566910] [INFO] Thread B (arg=2) ticks=43821539619
[44280768477] [INFO] Thread A (arg=1) ticks=44280735873
[44745038184] [INFO] Thread B (arg=2) ticks=44745010233
[45173682081] [INFO] Thread A (arg=1) ticks=45173653833
[45765732606] [INFO] Thread B (arg=2) ticks=45765705909
[46285784292] [INFO] Thread A (arg=1) ticks=46285755120
[46766226408] [INFO] Thread B (arg=2) ticks=46766197500
[47317329939] [INFO] Thread A (arg=1) ticks=47317303836
[47931426609] [INFO] Thread B (arg=2) ticks=47931397536
[48367265199] [INFO] Thread A (arg=1) ticks=48367236819
[48799749834] [INFO] Thread B (arg=2) ticks=48799722741

```
</details>
