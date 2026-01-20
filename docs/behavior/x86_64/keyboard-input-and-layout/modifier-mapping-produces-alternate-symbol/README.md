# ✅ Scenario: Modifier mapping produces alternate symbol

> Last run: 2026-01-19 21:17:21

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the clock window is ticking | ✅ | 36131ms | <a href="./01/after.png"><img src="./01/after.png" width="150" /></a> - [💾](./01/registers.txt) |
| 2 | When I press Alt+A | ✅ | 709ms | <a href="./02/after.png"><img src="./02/after.png" width="150" /></a> - [💾](./02/registers.txt) |
| 3 | Then the serial log should contain 'CONTRACT: input key_event' | ✅ | 448ms | <a href="./03/after.png"><img src="./03/after.png" width="150" /></a> - [💾](./03/registers.txt) |
| 4 | And I should see the appropriate symbol rendered | ✅ | 445ms | <a href="./04/after.png"><img src="./04/after.png" width="150" /></a> - [💾](./04/registers.txt) |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01H[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[10188927177] [CONTRACT] [kernel] thing-os kernel starting...
[10423648752] [CONTRACT] [kernel::memory] Frame allocator initialized with 495746 free frames
[10443158187] [CONTRACT] [kernel] Initializing global allocator...
[10793097678] [CONTRACT] [kernel] Initializing SIMD...
[10794994419] [CONTRACT] [kernel] Initializing tasking...
[10815263415] [CONTRACT] [kernel::task::scheduler] Scheduler initialized
[11867007372] [CONTRACT] [kernel] KERNEL: root census complete: host=t1 kernel=t3 root=t4
[11897789046] [CONTRACT] [kernel] Spawning init process...
[11933815311] [CONTRACT] [kernel] Entering scheduler loop.
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600000
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
USER_TRAMPOLINE: PC=0x2083b0 SP=0x800000 ARG0=0x0
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x0
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x30002
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xd1
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x5
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x7
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0x600080009000b
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xe0
USER_TRAMPOLINE: PC=0x200000 SP=0x800000 ARG0=0xc

```
</details>
