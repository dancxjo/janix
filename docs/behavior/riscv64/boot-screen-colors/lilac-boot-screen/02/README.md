# ✅ When I wait for the system to boot

**Result:** passed | **Duration:** 8929ms

## Screenshots

### After
![After](./after.png)

## Registers

```

CPU#0
 V      =   0
 pc       ffffffff800088b8
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
 mepc     ffffffff80003614
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
 satp     90000000000fab2a
 x0/zero  0000000000000000 x1/ra    ffffffff800086f8 x2/sp    ffff8000fab3ab20 x3/gp    0000000000000000
 x4/tp    0000000000000000 x5/t0    0000000000010000 x6/t1    ffffffff800098f2 x7/t2    0000000000002710
 x8/s0    0000000000000000 x9/s1    0000000000000000 x10/a0   00000000000003b8 x11/a1   0000000000000001
 x12/a2   00000000000003bb x13/a3   ffff8000fab3abc8 x14/a4   0000000000000000 x15/a5   0000000000000000
 x16/a6   ffffffff8000c2a8 x17/a7   0000000000010100 x18/s2   0000000000000000 x19/s3   0000000000000000
 x20/s4   0000000000000000 x21/s5   0000000000000000 x22/s6   0000000000000000 x23/s7   0000000000000000
 x24/s8   0000000000000000 x25/s9   0000000000000000 x26/s10  0000000000000000 x27/s11  0000000000000000
 x28/t3   000000000000147b x29/t4   000000000098967f x30/t5   ffffffff8000ef08 x31/t6   0000000000000000
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
[2J[04D[=3h[2J[09D[2J[04D[8;031;100t0[2J[17DRISC-V EDK2 firmware version 2.7
Press ESCAPE within 5 seconds for boot options [2J[01;01H[2J[04D[89528592] [INFO] thing-os kernel v0.1.0 starting...
[89568666] [INFO] Intent-Mechanism paging split active
[89588263] [INFO] System booted
[89611909] [INFO] Memory map has 24 entries
[89624286] [INFO]   [0] 0x22000000 - 0x24000000 (Reserved)
[89630370] [INFO]   [1] 0x80000000 - 0x80050000 (Reserved)
[89635075] [INFO]   [2] 0x80050000 - 0x83278000 (Usable)
[89639646] [INFO]   [3] 0x83278000 - 0x83fff000 (Reserved)
[89644306] [INFO]   [4] 0x83fff000 - 0xfaaeb000 (Usable)
[89648759] [INFO]   [5] 0xfaaeb000 - 0xfab43000 (Reserved)
[89659722] [INFO]   [6] 0xfab43000 - 0xfab58000 (Other)
[89679052] [INFO]   [7] 0xfab58000 - 0xfad70000 (Reserved)
[89699334] [INFO]   [8] 0xfad70000 - 0xfed6b000 (Usable)
[89714896] [INFO]   [9] 0xfed6b000 - 0xff1b6000 (Reserved)
[89720559] [INFO] HHDM Offset: 0xffff800000000000

```
</details>
