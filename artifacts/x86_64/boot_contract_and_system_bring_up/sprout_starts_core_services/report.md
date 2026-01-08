# Scenario: Sprout starts core services

**Architecture**: `x86_64`  
**Feature**: `Boot contract and system bring-up`

[Back to Index](../../../../index.md)

## Steps Summary

| Index | Step | Status | Artifacts |
| :---: | --- | :---: | --- |
| 0 | sprout is online | ✅ | �� 📝  |
| 1 | the graph should contain a Thing named "inputd" | ✅ | �� 📝  |
| 2 | the graph should contain a Thing named "logview" | ✅ | �� 📝  |

## Execution Details

### 1. sprout is online ✅

![Screenshot](steps/000_sprout_is_online/screen.png)

```
[36815.676 ms] USER: DRAIN[2]: watch_ids len=1
[36816.383 ms] USER: DRAIN[2]: polling watch 1
[36817.137 ms] USER: DRAIN[2]: done total=0
[36817.744 ms] USER: BLOOM STEP: after watches.drain n=0
[36889.186 ms] USER: BLOOM LOOP: iter=3
[36890.161 ms] USER: BLOOM STEP: after monotonic_now
[36890.792 ms] USER: BLOOM STEP: after wallpaper check
[36891.853 ms] USER: DRAIN[3]: enter
[36892.923 ms] USER: DRAIN[3]: scratch created
[36893.909 ms] USER: DRAIN[3]: watch_ids len=1
[36894.600 ms] USER: DRAIN[3]: polling watch 1
[36895.403 ms] USER: DRAIN[3]: done total=0
[36896.073 ms] USER: BLOOM STEP: after watches.drain n=0
[37491.652 ms] USER: CLOCK: 04:45:02
[38072.725 ms] USER: BLOOM DIAG: wp_started=false wp_phase=0 inp_started=1 inp_ticks=43603 events_drained=0
[38520.243 ms] USER: CLOCK: 04:45:03
[39542.152 ms] USER: CLOCK: 04:45:04
[40122.117 ms] USER: BLOOM DIAG: wp_started=false wp_phase=0 inp_started=1 inp_ticks=64298 events_drained=0
[40565.211 ms] USER: CLOCK: 04:45:05
[41590.875 ms] USER: CLOCK: 04:45:06
[42172.611 ms] USER: BLOOM DIAG: wp_started=false wp_phase=0 inp_started=1 inp_ticks=83501 events_drained=0
[42619.285 ms] USER: CLOCK: 04:45:07
[43640.256 ms] USER: CLOCK: 04:45:08
[44220.303 ms] USER: BLOOM DIAG: wp_started=false wp_phase=0 inp_started=1 inp_ticks=101290 events_drained=0
[44663.534 ms] USER: CLOCK: 04:45:09
[45688.882 ms] USER: CLOCK: 04:45:10
[46270.166 ms] USER: BLOOM DIAG: wp_started=false wp_phase=0 inp_started=1 inp_ticks=129515 events_drained=0
[46717.619 ms] USER: CLOCK: 04:45:11
[47738.782 ms] USER: CLOCK: 04:45:12
[48318.100 ms] USER: BLOOM DIAG: wp_started=false wp_phase=0 inp_started=1 inp_ticks=154546 events_drained=0
[48763.327 ms] USER: CLOCK: 04:45:13
[49786.420 ms] USER: CLOCK: 04:45:14
[50367.698 ms] USER: BLOOM DIAG: wp_started=false wp_phase=0 inp_started=1 inp_ticks=178952 events_drained=0
[50812.053 ms] USER: CLOCK: 04:45:15
[51836.752 ms] USER: CLOCK: 04:45:16
[52428.178 ms] USER: BLOOM DIAG: wp_started=false wp_phase=0 inp_started=1 inp_ticks=203352 events_drained=0
[52870.373 ms] USER: CLOCK: 04:45:17
[53895.085 ms] USER: CLOCK: 04:45:18
[54481.855 ms] USER: BLOOM DIAG: wp_started=false wp_phase=0 inp_started=1 inp_ticks=221441 events_drained=0
[54920.931 ms] USER: CLOCK: 04:45:19
[55946.930 ms] USER: CLOCK: 04:45:20
[56527.877 ms] USER: BLOOM DIAG: wp_started=false wp_phase=0 inp_started=1 inp_ticks=244720 events_drained=0
[56971.262 ms] USER: CLOCK: 04:45:21
[57993.599 ms] USER: CLOCK: 04:45:22
[58585.237 ms] USER: BLOOM DIAG: wp_started=false wp_phase=0 inp_started=1 inp_ticks=268169 events_drained=0
[59029.565 ms] USER: CLOCK: 04:45:23
[60055.863 ms] USER: CLOCK: 04:45:25
[60635.655 ms] USER: BLOOM DIAG: wp_started=false wp_phase=0 inp_started=1 inp_ticks=289734 events_drained=0
[61078.851 ms] USER: CLOCK: 04:45:26
[62102.174 ms] USER: CLOCK: 04:45:27
```

---

### 2. the graph should contain a Thing named "inputd" ✅

![Screenshot](steps/001_the_graph_should_contain_a_thing_named__inputd_/screen.png)

```
[57993.599 ms] USER: CLOCK: 04:45:22
[58585.237 ms] USER: BLOOM DIAG: wp_started=false wp_phase=0 inp_started=1 inp_ticks=268169 events_drained=0
[59029.565 ms] USER: CLOCK: 04:45:23
[60055.863 ms] USER: CLOCK: 04:45:25
[60635.655 ms] USER: BLOOM DIAG: wp_started=false wp_phase=0 inp_started=1 inp_ticks=289734 events_drained=0
[61078.851 ms] USER: CLOCK: 04:45:26
[62102.174 ms] USER: CLOCK: 04:45:27
[62685.354 ms] USER: BLOOM DIAG: wp_started=false wp_phase=0 inp_started=1 inp_ticks=308809 events_drained=0
[63127.553 ms] USER: CLOCK: 04:45:28
[64152.356 ms] USER: CLOCK: 04:45:29
[64732.258 ms] USER: BLOOM DIAG: wp_started=false wp_phase=0 inp_started=1 inp_ticks=334661 events_drained=0
[65176.369 ms] USER: CLOCK: 04:45:30
[66204.261 ms] USER: CLOCK: 04:45:31
[66782.071 ms] USER: BLOOM DIAG: wp_started=false wp_phase=0 inp_started=1 inp_ticks=361161 events_drained=0
[67228.261 ms] USER: CLOCK: 04:45:32
[68249.950 ms] USER: CLOCK: 04:45:33
[68829.815 ms] USER: BLOOM DIAG: wp_started=false wp_phase=0 inp_started=1 inp_ticks=392266 events_drained=0
[69274.299 ms] USER: CLOCK: 04:45:34
[70298.333 ms] USER: CLOCK: 04:45:35
[70879.035 ms] USER: BLOOM DIAG: wp_started=false wp_phase=0 inp_started=1 inp_ticks=437628 events_drained=0
[71323.934 ms] USER: CLOCK: 04:45:36
[72347.661 ms] USER: CLOCK: 04:45:37
[72929.773 ms] USER: BLOOM DIAG: wp_started=false wp_phase=0 inp_started=1 inp_ticks=475394 events_drained=0
[73372.549 ms] USER: CLOCK: 04:45:38
[74396.486 ms] USER: CLOCK: 04:45:39
[74977.278 ms] USER: BLOOM DIAG: wp_started=false wp_phase=0 inp_started=1 inp_ticks=516858 events_drained=0
[75421.108 ms] USER: CLOCK: 04:45:40
[76445.776 ms] USER: CLOCK: 04:45:41
[77027.882 ms] USER: BLOOM DIAG: wp_started=false wp_phase=0 inp_started=1 inp_ticks=559203 events_drained=0
[77471.555 ms] USER: CLOCK: 04:45:42
[78494.999 ms] USER: CLOCK: 04:45:43
[79076.072 ms] USER: BLOOM DIAG: wp_started=false wp_phase=0 inp_started=1 inp_ticks=601351 events_drained=0
[79520.364 ms] USER: CLOCK: 04:45:44
[80544.816 ms] USER: CLOCK: 04:45:45
[81125.715 ms] USER: BLOOM DIAG: wp_started=false wp_phase=0 inp_started=1 inp_ticks=641230 events_drained=0
[81570.679 ms] USER: CLOCK: 04:45:46
[82594.342 ms] USER: CLOCK: 04:45:47
[83174.497 ms] USER: BLOOM DIAG: wp_started=false wp_phase=0 inp_started=1 inp_ticks=675734 events_drained=0
[83618.740 ms] USER: CLOCK: 04:45:48
[84642.565 ms] USER: CLOCK: 04:45:49
[85224.475 ms] USER: BLOOM DIAG: wp_started=false wp_phase=0 inp_started=1 inp_ticks=722883 events_drained=0
[85667.235 ms] USER: CLOCK: 04:45:50
[86696.499 ms] USER: CLOCK: 04:45:51
[87274.286 ms] USER: BLOOM DIAG: wp_started=false wp_phase=0 inp_started=1 inp_ticks=748807 events_drained=0
[87718.296 ms] USER: CLOCK: 04:45:52
[88741.621 ms] USER: CLOCK: 04:45:53
[89322.123 ms] USER: BLOOM DIAG: wp_started=false wp_phase=0 inp_started=1 inp_ticks=781326 events_drained=0
[89776.324 ms] USER: CLOCK: 04:45:54
[90801.860 ms] USER: CLOCK: 04:45:55
[91381.304 ms] USER: BLOOM DIAG: wp_started=false wp_phase=0 inp_started=1 inp_ticks=804151 events_drained=0
```

---

### 3. the graph should contain a Thing named "logview" ✅

![Screenshot](steps/002_the_graph_should_contain_a_thing_named__logview_/screen.png)

```
[85224.475 ms] USER: BLOOM DIAG: wp_started=false wp_phase=0 inp_started=1 inp_ticks=722883 events_drained=0
[85667.235 ms] USER: CLOCK: 04:45:50
[86696.499 ms] USER: CLOCK: 04:45:51
[87274.286 ms] USER: BLOOM DIAG: wp_started=false wp_phase=0 inp_started=1 inp_ticks=748807 events_drained=0
[87718.296 ms] USER: CLOCK: 04:45:52
[88741.621 ms] USER: CLOCK: 04:45:53
[89322.123 ms] USER: BLOOM DIAG: wp_started=false wp_phase=0 inp_started=1 inp_ticks=781326 events_drained=0
[89776.324 ms] USER: CLOCK: 04:45:54
[90801.860 ms] USER: CLOCK: 04:45:55
[91381.304 ms] USER: BLOOM DIAG: wp_started=false wp_phase=0 inp_started=1 inp_ticks=804151 events_drained=0
[91826.928 ms] USER: CLOCK: 04:45:56
[92849.343 ms] USER: CLOCK: 04:45:57
[93431.221 ms] USER: BLOOM DIAG: wp_started=false wp_phase=0 inp_started=1 inp_ticks=834112 events_drained=0
[93873.996 ms] USER: CLOCK: 04:45:58
[94899.126 ms] USER: CLOCK: 04:45:59
[95480.175 ms] USER: BLOOM DIAG: wp_started=false wp_phase=0 inp_started=1 inp_ticks=873019 events_drained=0
[95924.109 ms] USER: CLOCK: 04:46:00
[96948.476 ms] USER: CLOCK: 04:46:01
[97529.810 ms] USER: BLOOM DIAG: wp_started=false wp_phase=0 inp_started=1 inp_ticks=915080 events_drained=0
[97972.383 ms] USER: CLOCK: 04:46:02
[98997.068 ms] USER: CLOCK: 04:46:03
[99577.778 ms] USER: BLOOM DIAG: wp_started=false wp_phase=0 inp_started=1 inp_ticks=971802 events_drained=0
[100021.545 ms] USER: CLOCK: 04:46:04
[101046.391 ms] USER: CLOCK: 04:46:06
[101628.150 ms] USER: BLOOM DIAG: wp_started=false wp_phase=0 inp_started=1 inp_ticks=1030262 events_drained=0
[102070.834 ms] USER: CLOCK: 04:46:07
[103095.697 ms] USER: CLOCK: 04:46:08
[103676.157 ms] USER: BLOOM DIAG: wp_started=false wp_phase=0 inp_started=1 inp_ticks=1080164 events_drained=0
[104120.092 ms] USER: CLOCK: 04:46:09
[105144.621 ms] USER: CLOCK: 04:46:10
[105725.344 ms] USER: BLOOM DIAG: wp_started=false wp_phase=0 inp_started=1 inp_ticks=1137527 events_drained=0
[106169.324 ms] USER: CLOCK: 04:46:11
[107193.879 ms] USER: CLOCK: 04:46:12
[107774.589 ms] USER: BLOOM DIAG: wp_started=false wp_phase=0 inp_started=1 inp_ticks=1190630 events_drained=0
[108219.468 ms] USER: CLOCK: 04:46:13
[109242.983 ms] USER: CLOCK: 04:46:14
[109823.826 ms] USER: BLOOM DIAG: wp_started=false wp_phase=0 inp_started=1 inp_ticks=1241319 events_drained=0
[110267.704 ms] USER: CLOCK: 04:46:15
[111292.756 ms] USER: CLOCK: 04:46:16
[111873.076 ms] USER: BLOOM DIAG: wp_started=false wp_phase=0 inp_started=1 inp_ticks=1297480 events_drained=0
[112317.173 ms] USER: CLOCK: 04:46:17
[113341.513 ms] USER: CLOCK: 04:46:18
[113922.239 ms] USER: BLOOM DIAG: wp_started=false wp_phase=0 inp_started=1 inp_ticks=1342499 events_drained=0
[114366.130 ms] USER: CLOCK: 04:46:19
[115390.821 ms] USER: CLOCK: 04:46:20
[115971.708 ms] USER: BLOOM DIAG: wp_started=false wp_phase=0 inp_started=1 inp_ticks=1395829 events_drained=0
[116415.736 ms] USER: CLOCK: 04:46:21
[117439.914 ms] USER: CLOCK: 04:46:22
[118020.671 ms] USER: BLOOM DIAG: wp_started=false wp_phase=0 inp_started=1 inp_ticks=1453680 events_drained=0
[118464.562 ms] USER: CLOCK: 04:46:23
```

---

