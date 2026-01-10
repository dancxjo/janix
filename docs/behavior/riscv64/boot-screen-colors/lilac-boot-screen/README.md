# ✅ Scenario: Lilac Boot Screen

> Last run: 2026-01-10 15:26:13

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is started | ✅ | 572ms | <a href="./01/after.png"><img src="./01/after.png" width="150" /></a> - [💾](./01/registers.txt) |
| 2 | When I wait for the system to boot | ✅ | 8329ms | <a href="./02/after.png"><img src="./02/after.png" width="150" /></a> [📜](./02/serial.log) [💾](./02/registers.txt) |
| 3 | Then the screen should be filled with "Lilac" | ✅ | 932ms | <a href="./03/after.png"><img src="./03/after.png" width="150" /></a> - [💾](./03/registers.txt) |

<details>
<summary>📜 Full Serial Log</summary>

```

OpenSBI v1.7
   ____                    _____ ____ _____
  / __ \                  / ____|  _ \_   _|
 | |  | |_ __   ___ _ __ | (___ | |_) || |
 | |  | | '_ \ / _ \ '_ \ \___ \|  _ < | |
 | |__| | |_) |  __/ | | |____) | |_) || |_
  \____/| .__/ \___|_| |_|_____/|____/_____|
        | |
        |_|

Platform Name               : riscv-virtio,qemu
Platform Features           : medeleg
Platform HART Count         : 1
Platform IPI Device         : aclint-mswi
Platform Timer Device       : aclint-mtimer @ 10000000Hz
Platform Console Device     : uart8250
Platform HSM Device         : ---
Platform PMU Device         : ---
Platform Reboot Device      : syscon-reboot
Platform Shutdown Device    : syscon-poweroff
Platform Suspend Device     : ---
Platform CPPC Device        : ---
Firmware Base               : 0x80000000
Firmware Size               : 317 KB
Firmware RW Offset          : 0x40000
Firmware RW Size            : 61 KB
Firmware Heap Offset        : 0x46000
Firmware Heap Size          : 37 KB (total), 2 KB (reserved), 11 KB (used), 23 KB (free)
Firmware Scratch Size       : 4096 B (total), 1400 B (used), 2696 B (free)
Runtime SBI Version         : 3.0
Standard SBI Extensions     : ipi,pmu,srst,sse,hsm,rfnc,fwft,time,base,legacy,dbcn,dbtr
Experimental SBI Extensions : none

Domain0 Name                : root
Domain0 Boot HART           : 0
Domain0 HARTs               : 0*
Domain0 Region00            : 0x0000000000100000-0x0000000000100fff M: (I,R,W) S/U: (R,W)
Domain0 Region01            : 0x0000000010000000-0x0000000010000fff M: (I,R,W) S/U: (R,W)
Domain0 Region02            : 0x0000000002000000-0x000000000200ffff M: (I,R,W) S/U: ()
Domain0 Region03            : 0x0000000080040000-0x000000008004ffff M: (R,W) S/U: ()
Domain0 Region04            : 0x0000000080000000-0x000000008003ffff M: (R,X) S/U: ()
Domain0 Region05            : 0x000000000c400000-0x000000000c5fffff M: (I,R,W) S/U: (R,W)
Domain0 Region06            : 0x000000000c000000-0x000000000c3fffff M: (I,R,W) S/U: (R,W)
Domain0 Region07            : 0x0000000000000000-0xffffffffffffffff M: () S/U: (R,W,X)
Domain0 Next Address        : 0x0000000020000000
Domain0 Next Arg1           : 0x00000000ffe00000
Domain0 Next Mode           : S-mode
Domain0 SysReset            : yes
Domain0 SysSuspend          : yes

Boot HART ID                : 0
Boot HART Domain            : root
Boot HART Priv Version      : v1.12
Boot HART Base ISA          : rv64imafdch
Boot HART ISA Extensions    : sstc,zicntr,zihpm,zicboz,zicbom,sdtrig,svadu
Boot HART PMP Count         : 16
Boot HART PMP Granularity   : 2 bits
Boot HART PMP Address Bits  : 54
Boot HART MHPM Info         : 16 (0x0007fff8)
Boot HART Debug Triggers    : 2 triggers
Boot HART MIDELEG           : 0x0000000000001666
Boot HART MEDELEG           : 0x0000000000f4b509
[2J[04D[=3h[2J[09D[2J[04D[8;031;100t0[2J[17DRISC-V EDK2 firmware version 2.7
Press ESCAPE within 5 seconds for boot options [2J[01;01H[2J[04D[85647313] [INFO] thing-os kernel v0.1.0 starting...
[85677902] [INFO] Intent-Mechanism paging split active
[85692584] [INFO] System booted
[85708347] [INFO] Memory map has 24 entries
[85725937] [INFO]   [0] 0x22000000 - 0x24000000 (Reserved)
[85738519] [INFO]   [1] 0x80000000 - 0x80050000 (Reserved)
[85749071] [INFO]   [2] 0x80050000 - 0x83278000 (Usable)
[85758173] [INFO]   [3] 0x83278000 - 0x83fff000 (Reserved)
[85768765] [INFO]   [4] 0x83fff000 - 0xfaaeb000 (Usable)
[85776761] [INFO]   [5] 0xfaaeb000 - 0xfab43000 (Reserved)
[85788924] [INFO]   [6] 0xfab43000 - 0xfab58000 (Other)
[85793911] [INFO]   [7] 0xfab58000 - 0xfad70000 (Reserved)
[85798283] [INFO]   [8] 0xfad70000 - 0xfed6b000 (Usable)
[85814041] [INFO]   [9] 0xfed6b000 - 0xff1b6000 (Reserved)
[85825552] [INFO] HHDM Offset: 0xffff800000000000
[88784930] [INFO] Frame allocator initialized with 515311 free frames
[88805421] [INFO] Initializing global allocator...

```
</details>
