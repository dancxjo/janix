# ❌ Scenario: Threads A and B interleave

> Last run: 2026-01-10 11:21:55

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 9447ms | <a href="./01/after.png"><img src="./01/after.png" width="150" /></a> [📜](./01/serial.log) [💾](./01/registers.txt) |
| 2 | Then I should see "Spawning Thread A..." | ✅ | 1399ms | <a href="./02/after.png"><img src="./02/after.png" width="150" /></a> [📜](./02/serial.log) [💾](./02/registers.txt) |
| 3 | And I should see "Spawning Thread B..." | ✅ | 2136ms | <a href="./03/after.png"><img src="./03/after.png" width="150" /></a> [📜](./03/serial.log) [💾](./03/registers.txt) |
| 4 | And I should see "Thread A (arg=1) ticks=" | ✅ | 1218ms | <a href="./04/after.png"><img src="./04/after.png" width="150" /></a> [📜](./04/serial.log) [💾](./04/registers.txt) |
| 5 | And "Thread A (arg=1) ticks=" should appear at least 2 times | ✅ | 1085ms | <a href="./05/after.png"><img src="./05/after.png" width="150" /></a> [📜](./05/serial.log) [💾](./05/registers.txt) |
| 6 | And I should see "Thread B (arg=2) ticks=" | ✅ | 1571ms | <a href="./06/after.png"><img src="./06/after.png" width="150" /></a> [📜](./06/serial.log) [💾](./06/registers.txt) |
| 7 | And "Thread B (arg=2) ticks=" should appear at least 2 times | ✅ | 1952ms | <a href="./07/after.png"><img src="./07/after.png" width="150" /></a> [📜](./07/serial.log) [💾](./07/registers.txt) |
| 8 | And I should see "Thread A (arg=1) ticks=" after "Thread B (arg=2) ticks=" | ❌ | 20ms | - [📜](./08/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[24451765140] [INFO] thing-os kernel v0.1.0 starting...
[24474088584] [INFO] Intent-Mechanism paging split active
[24474575664] [INFO] System booted
[24486172755] [INFO] Memory map has 35 entries
[24491499153] [INFO]   [0] 0x0 - 0xa0000 (Usable)
[24492394608] [INFO]   [1] 0x100000 - 0x800000 (Usable)
[24492808890] [INFO]   [2] 0x800000 - 0x808000 (Other)
[24498883233] [INFO]   [3] 0x808000 - 0x80b000 (Usable)
[24503239266] [INFO]   [4] 0x80b000 - 0x80c000 (Other)
[24503644671] [INFO]   [5] 0x80c000 - 0x811000 (Usable)
[24504053145] [INFO]   [6] 0x811000 - 0x900000 (Other)
[24504453435] [INFO]   [7] 0x900000 - 0x1780000 (Reserved)
[24505220520] [INFO]   [8] 0x1780000 - 0x79fa6000 (Usable)
[24505652721] [INFO]   [9] 0x79fa6000 - 0x7a16c000 (Reserved)
[24510259818] [INFO] HHDM Offset: 0xffff800000000000
[25964328060] [INFO] Frame allocator initialized with 511944 free frames
[26017143141] [INFO] Spawning Thread A...
[26036912088] [INFO] Spawning Thread B...
[26049995334] [INFO] System initialized. Entering scheduler loop.
[26071103883] [INFO] Thread A (arg=1) ticks=26070850740
[26958442896] [INFO] Thread B (arg=2) ticks=26958212589
[28173769173] [INFO] Thread A (arg=1) ticks=28173738516
[29558793396] [INFO] Thread B (arg=2) ticks=29558763366
[30008779284] [INFO] Thread A (arg=1) ticks=30008749914
[30853358712] [INFO] Thread B (arg=2) ticks=30850004460
[31706346045] [INFO] Thread A (arg=1) ticks=31706318589
[33157781514] [INFO] Thread B (arg=2) ticks=33157750494
[34739614404] [INFO] Thread A (arg=1) ticks=34739585628
[35559270714] [INFO] Thread B (arg=2) ticks=35559240717
[36418931406] [INFO] Thread A (arg=1) ticks=36418903257
[37294764309] [INFO] Thread B (arg=2) ticks=37294733982
[37887574296] [INFO] Thread A (arg=1) ticks=37887542979
[39026587479] [INFO] Thread B (arg=2) ticks=39026558538
[39756857547] [INFO] Thread A (arg=1) ticks=39756830355
[40627894989] [INFO] Thread B (arg=2) ticks=40627859976
[41234504982] [INFO] Thread A (arg=1) ticks=41234474358
[42060621768] [INFO] Thread B (arg=2) ticks=42060587712
[42869745633] [INFO] Thread A (arg=1) ticks=42869715735
[44421178131] [INFO] Thread B (arg=2) ticks=44421142392
[46161969876] [INFO] Thread A (arg=1) ticks=46161941562
[47893406814] [INFO] Thread B (arg=2) ticks=47893371636
[49257641895] [INFO] Thread A (arg=1) ticks=49257608037
[50278698987] [INFO] Thread B (arg=2) ticks=50278668198
[51566924904] [INFO] Thread A (arg=1) ticks=51566893026
[52712786973] [INFO] Thread B (arg=2) ticks=52712757042
[53120194545] [INFO] Thread A (arg=1) ticks=53120164449
[53675459244] [INFO] Thread B (arg=2) ticks=53675426739
[54464292564] [INFO] Thread A (arg=1) ticks=54464259630
[55245921225] [INFO] Thread B (arg=2) ticks=55245882219
[55960063500] [INFO] Thread A (arg=1) ticks=55960026210
[57343027797] [INFO] Thread B (arg=2) ticks=57342999417
[58406806623] [INFO] Thread A (arg=1) ticks=58406781642
[59256168081] [INFO] Thread B (arg=2) ticks=59256134058
[59857855167] [INFO] Thread A (arg=1) ticks=59857822002
[61334840985] [INFO] Thread B (arg=2) ticks=61334810691
[62547537294] [INFO] Thread A (arg=1) ticks=62547508188
[62973690252] [INFO] Thread B (arg=2) ticks=62973658275
[63488376072] [INFO] Thread A (arg=1) ticks=63488346042
[63974827851] [INFO] Thread B (arg=2) ticks=63974795346
[64594528119] [INFO] Thread A (arg=1) ticks=64594494822
[65154375495] [INFO] Thread B (arg=2) ticks=65154344211
[65677477635] [INFO] Thread A (arg=1) ticks=65677450839
[67925857824] [INFO] Thread B (arg=2) ticks=67925825088
[69471975078] [INFO] Thread A (arg=1) ticks=69471947589
[70430815851] [INFO] Thread B (arg=2) ticks=70430787570
[71662366875] [INFO] Thread A (arg=1) ticks=71662333215
[72479519211] [INFO] Thread B (arg=2) ticks=72479490039
[73273398858] [INFO] Thread A (arg=1) ticks=73273371072
[74270854251] [INFO] Thread B (arg=2) ticks=74270824419
[75051900726] [INFO] Thread A (arg=1) ticks=75051871851
[75523691991] [INFO] Thread B (arg=2) ticks=75523659816
[76228821174] [INFO] Thread A (arg=1) ticks=76228796259
[77532628503] [INFO] Thread B (arg=2) ticks=77532597483
[78663158607] [INFO] Thread A (arg=1) ticks=78663131415
[79583632590] [INFO] Thread B (arg=2) ticks=79583603880
[80538090138] [INFO] Thread A (arg=1) ticks=80538062517
[81541338615] [INFO] Thread B (arg=2) ticks=81541308486
[82504584591] [INFO] Thread A (arg=1) ticks=82504558422
[83566614186] [INFO] Thread B (arg=2) ticks=83566577292
[84784837434] [INFO] Thread A (arg=1) ticks=84784808097
[85876425492] [INFO] Thread B (arg=2) ticks=85876395759
[86802088461] [INFO] Thread A (arg=1) ticks=86802059025
[87745027821] [INFO] Thread B (arg=2) ticks=87744996669
[88751992626] [INFO] Thread A (arg=1) ticks=88751960946
[89679806040] [INFO] Thread B (arg=2) ticks=89679776604
[90596830236] [INFO] Thread A (arg=1) ticks=90596804133
[91496246919] [INFO] Thread B (arg=2) ticks=91496215074

```
</details>
