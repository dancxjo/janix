# ✅ Scenario: Heap Growth on Demand

> Last run: 2026-01-09 10:57:12

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booting | ⏭️ | 3ms | - - - |

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
Press ESCAPE within 5 seconds for boot options [2J[01;01HUEFI Interactive Shell v2.2
EDK II
UEFI v2.70 (EDK II, 0x00010000)
[1m[33m[40mMapping table[0m[37m[40m
[1m[33m[40m      FS0:[0m[37m[40m [1m[37m[40mAlias(s):[0m[37m[40mHD0c:;BLK2:
          VenHw(837DCA9E-E874-4D82-B29A-23FE0E23D1E2,0080001000000000)/HD(2,GPT,5588D5D0-1695-4BCC-B
578-02EB9E58D87E)
[1m[33m[40m     BLK0:[0m[37m[40m [1m[37m[40mAlias(s):[0m[37m[40m
          VenHw(837DCA9E-E874-4D82-B29A-23FE0E23D1E2,0080001000000000)
[1m[33m[40m     BLK1:[0m[37m[40m [1m[37m[40mAlias(s):[0m[37m[40m
          VenHw(837DCA9E-E874-4D82-B29A-23FE0E23D1E2,0080001000000000)/HD(1,GPT,5588D5D0-1695-4BCC-B
57B-02EB9E58D87E)
[1m[33m[40m     BLK3:[0m[37m[40m [1m[37m[40mAlias(s):[0m[37m[40m
          VenHw(837DCA9E-E874-4D82-B29A-23FE0E23D1E2,0080001000000000)/HD(3,GPT,5588D5D0-1695-4BCC-B
579-02EB9E58D87E)
Press [1m[37m[40mESC[0m[37m[40m in 5 seconds to skip [1m[33m[40mstartup.nsh[0m[37m[40m or any other key to continue.[72DPress [1m[37m[40mESC[0m[37m[40m in 4 seconds to skip [1m[33m[40mstartup.nsh[0m[37m[40m or any other key to continue.[72DPress [1m[37m[40mESC[0m[37m[40m in 3 seconds to skip [1m[33m[40mstartup.nsh[0m[37m[40m or any other key to continue.[72DPress [1m[37m[40mESC[0m[37m[40m in 2 seconds to skip [1m[33m[40mstartup.nsh[0m[37m[40m or any other key to continue.[72DPress [1m[37m[40mESC[0m[37m[40m in 1 seconds to skip [1m[33m[40mstartup.nsh[0m[37m[40m or any other key to continue.
[1m[33m[40mShell> [0m[37m[40m\EFI\BOOT\BOOTRISCV64.EFI
[2J[01;01H[2J[04D[100591652] [INFO] System booted
[100651559] [INFO] boot: phys ranges=24 modules=0
[100672852] [INFO] BootHeap initialized. Range: 0xffffff8040000000 - 0xffffff8041000000
[100790723] [INFO] Initializing Real Frame Allocator...
[100807780] [INFO] frame_alloc: base=0x80050000 frames=519021 words=8110
[100826476] [INFO] Allocating bitmap of 8110 words...
[100855648] [INFO] Bitmap allocated at 0xffffff8040000000
[100875692] [INFO] Zeroing bitmap...

```
</details>
