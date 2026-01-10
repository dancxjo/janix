# ✅ Given the machine is booted

**Result:** passed | **Duration:** 8310ms

## Screenshots

### After
![After](./after.png)

## Registers

```

CPU#0
 V      =   0
 pc       ffffffff80007f42
 mhartid  0000000000000000
 mstatus  8000000a00006180
 hstatus  0000000200000000
 vsstatus 0000000a00000000
 mip      0000000000000020
 mie      0000000000000088
 mideleg  0000000000001666
 hideleg  0000000000000000
 medeleg  0000000000f4b509
 hedeleg  0000000000000000
 mtvec    00000000800004f8
 stvec    0000000000000000
 vstvec   0000000000000000
 mepc     ffffffff8000361c
 sepc     00000000fed9505a
 vsepc    0000000000000000
 mcause   0000000000000009
 scause   000000000000000c
 vscause  0000000000000000
 mtval    0000000000000000
 stval    00000000fed9505a
 htval    0000000000000000
 mtval2   0000000000000000
 mscratch 0000000080045000
 sscratch 0000000082000000
 satp     90000000000fab2b
 x0/zero  0000000000000000 x1/ra    ffffffff800085fe x2/sp    ffff8000fab3bb70 x3/gp    0000000000000000
 x4/tp    0000000000000000 x5/t0    0000000000010000 x6/t1    0000000000000064 x7/t2    0000000000002710
 x8/s0    0000000000000000 x9/s1    0000000000000000 x10/a0   00000000000011d4 x11/a1   0000000000008ea8
 x12/a2   00000000000011d6 x13/a3   ffff8000fab3bc68 x14/a4   0000000000000000 x15/a5   0000000000000000
 x16/a6   ffffffff8000c2a8 x17/a7   0000000000010100 x18/s2   0000000000000000 x19/s3   0000000000000000
 x20/s4   0000000000000000 x21/s5   0000000000000000 x22/s6   0000000000000000 x23/s7   0000000000000000
 x24/s8   0000000000000000 x25/s9   0000000000000000 x26/s10  0000000000000000 x27/s11  0000000000000000
 x28/t3   000000000000147b x29/t4   000000000098967f x30/t5   ffffffff8000ed48 x31/t6   0000000000000000
 fcsr     0000000000000000
 f0/ft0   ffffffff00000000 f1/ft1   ffffffff00000000 f2/ft2   ffffffff00000000 f3/ft3   ffffffff00000000
 f4/ft4   ffffffff00000000 f5/ft5   ffffffff00000000 f6/ft6   ffffffff00000000 f7/ft7   ffffffff00000000
 f8/fs0   ffffffff00000000 f9/fs1   ffffffff00000000 f10/fa0  ffffffff00000000 f11/fa1  ffffffff00000000
 f12/fa2  ffffffff00000000 f13/fa3  ffffffff00000000 f14/fa4  ffffffff00000000 f15/fa5  4040800000000000
 f16/fa6  ffffffff00000000 f17/fa7  ffffffff00000000 f18/fs2  ffffffff00000000 f19/fs3  ffffffff00000000
 f20/fs4  ffffffff00000000 f21/fs5  ffffffff00000000 f22/fs6  ffffffff00000000 f23/fs7  ffffffff00000000
 f24/fs8  ffffffff00000000 f25/fs9  ffffffff00000000 f26/fs10 ffffffff00000000 f27/fs11 ffffffff00000000
 f28/ft8  ffffffff00000000 f29/ft9  ffffffff00000000 f30/ft10 ffffffff00000000 f31/ft11 ffffffff00000000

```

<details open>
<summary>Serial Output</summary>

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
Press ESCAPE within 5 seconds for boot options [2J[01;01H[2J[04D[78727768] [INFO] thing-os kernel v0.1.0 starting...
[78750743] [INFO] Intent-Mechanism paging split active
[78755393] [INFO] System booted
[78769891] [INFO] Memory map has 24 entries
[78777954] [INFO]   [0] 0x22000000 - 0x24000000 (Reserved)
[78784033] [INFO]   [1] 0x80000000 - 0x80050000 (Reserved)
[78788620] [INFO]   [2] 0x80050000 - 0x83278000 (Usable)
[78793289] [INFO]   [3] 0x83278000 - 0x83fff000 (Reserved)
[78797997] [INFO]   [4] 0x83fff000 - 0xfaaec000 (Usable)
[78802576] [INFO]   [5] 0xfaaec000 - 0xfab44000 (Reserved)
[78807208] [INFO]   [6] 0xfab44000 - 0xfab59000 (Other)
[78811804] [INFO]   [7] 0xfab59000 - 0xfad70000 (Reserved)
[78816443] [INFO]   [8] 0xfad70000 - 0xfed6b000 (Usable)
[78821062] [INFO]   [9] 0xfed6b000 - 0xff1b6000 (Reserved)
[78826318] [INFO] HHDM Offset: 0xffff800000000000

```
</details>
