# Scenario: The Kernel exposes a root graph and a devices graph

**Architecture**: `x86_64`  
**Feature**: `Boot contract and system bring-up`

[Back to Index](../../../../index.md)

## Steps Summary

| Index | Step | Status | Artifacts |
| :---: | --- | :---: | --- |
| 0 | the system has reached "kernel ready" | ✅ | �� 📝  |
| 1 | I query the graph for the root graph | ⏭️ | �� 📝  |

## Execution Details

### 1. the system has reached "kernel ready" ✅

![Screenshot](steps/000_the_system_has_reached__kernel_ready_/screen.png)

```
[36260.390 ms] USER: TRACE: render_window_scenes: ENTER
[36262.073 ms] USER: TRACE: render_window_scenes: EXIT (1.460ms)
[36265.463 ms] USER: TRACE: scene_record: EXIT (79.181ms)
[36266.266 ms] USER: TRACE: exec_chunk: ENTER
[36418.084 ms] USER: TRACE: exec_chunk: EXIT (150.967ms)
[36466.509 ms] USER: CLOCK: 04:45:01
[36498.478 ms] USER: BLOOM: cursor overlay active with shadows
[36740.593 ms] USER: BLOOM LOOP: iter=2
[36811.426 ms] USER: BLOOM STEP: after monotonic_now
[36812.831 ms] USER: BLOOM STEP: after wallpaper check
[36814.110 ms] USER: DRAIN[2]: enter
[36814.927 ms] USER: DRAIN[2]: scratch created
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
```

---

### 2. I query the graph for the root graph ❌

![Screenshot](steps/001_i_query_the_graph_for_the_root_graph/screen.png)

```
[36266.266 ms] USER: TRACE: exec_chunk: ENTER
[36418.084 ms] USER: TRACE: exec_chunk: EXIT (150.967ms)
[36466.509 ms] USER: CLOCK: 04:45:01
[36498.478 ms] USER: BLOOM: cursor overlay active with shadows
[36740.593 ms] USER: BLOOM LOOP: iter=2
[36811.426 ms] USER: BLOOM STEP: after monotonic_now
[36812.831 ms] USER: BLOOM STEP: after wallpaper check
[36814.110 ms] USER: DRAIN[2]: enter
[36814.927 ms] USER: DRAIN[2]: scratch created
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
```

---

