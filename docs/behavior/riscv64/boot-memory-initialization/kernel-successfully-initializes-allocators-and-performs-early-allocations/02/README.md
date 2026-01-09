# ✅ Then the serial output should contain "boot: phys ranges="

**Result:** passed | **Duration:** 10400ms

## Screenshots

### After
![After](./after.png)

## Registers

```

CPU#0
 V      =   0
 pc       0000000000000000
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
 mepc     ffffffff8000114c
 sepc     0000000000000000
 vsepc    0000000000000000
 mcause   0000000000000009
 scause   000000000000000c
 vscause  0000000000000000
 mtval    0000000000000000
 stval    0000000000000000
 htval    0000000000000000
 mtval2   0000000000000000
 mscratch 0000000080045000
 sscratch 0000000082000000
 satp     90000000000fecd4
 x0/zero  0000000000000000 x1/ra    ffffffff80005f54 x2/sp    ffff8000fa9143c0 x3/gp    0000000000000000
 x4/tp    0000000000000000 x5/t0    346dc5d63886594b x6/t1    ffffffff8000c410 x7/t2    0000000000002710
 x8/s0    0000000000000000 x9/s1    0000000000000000 x10/a0   0000000000000000 x11/a1   ffffff8040000000
 x12/a2   000000000000fd70 x13/a3   0000000000000000 x14/a4   ffffffff80011710 x15/a5   ffffffff80004426
 x16/a6   ffff8000fa9141b8 x17/a7   0000000000000001 x18/s2   0000000000000000 x19/s3   0000000000000000
 x20/s4   0000000000000000 x21/s5   0000000000000000 x22/s6   0000000000000000 x23/s7   0000000000000000
 x24/s8   0000000000000000 x25/s9   0000000000000000 x26/s10  0000000000000000 x27/s11  0000000000000000
 x28/t3   000000000000147b x29/t4   000000000098967f x30/t5   ffffffff80012b52 x31/t6   0000000000000002
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
[2J[01;01H[2J[04D[108636152] [INFO] System booted
[108679905] [INFO] boot: phys ranges=24 modules=0
[108690689] [INFO] BootHeap initialized. Range: 0xffffff8040000000 - 0xffffff8041000000
[108716823] [INFO] Initializing Real Frame Allocator...
[108724567] [INFO] frame_alloc: base=0x80050000 frames=519021 words=8110
[108729999] [INFO] Allocating bitmap of 8110 words...
[108756621] [INFO] Bitmap allocated at 0xffffff8040000000
[108776443] [INFO] Zeroing bitmap...

```
</details>
