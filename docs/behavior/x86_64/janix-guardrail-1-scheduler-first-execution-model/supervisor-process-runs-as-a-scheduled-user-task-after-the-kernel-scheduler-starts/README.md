# ✅ Scenario: Supervisor process runs as a scheduled user task after the kernel scheduler starts

> Last run: 2026-04-12 15:53:37

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booting | ✅ | 0ms | - - - |
| 2 | Then I should see a message in the serial output that says "Entering scheduler loop" within 60s | ✅ | 8814ms | - - - |
| 3 | And I should see a message in the serial output that says "SPROUT:" within 120s | ✅ | 0ms | - - - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[21702733998] [[36m-----[0m] [kernel] [CPU0] thing-os kernel starting...
[22190101989] [[36m-----[0m] [bran::arch::x86_64] [CPU0] [SERIAL] UART COM1 initialized (115200 8N1 FIFO-1 IRQ-on)
[22202139696] [[36m-----[0m] [kernel] [CPU0] Initializing global allocator...
[27867069945] [[32mINFO [0m] [kernel::vfs::devfs] [CPU0] devfs: set_boot_fb width=1920 height=1080 pitch=7680 resource_id=0xfb000000
[27890120775] [[36m-----[0m] [kernel] [CPU0] Seeding entropy pool...
[27895508883] [[36m-----[0m] [kernel] [CPU0] Initializing SIMD...
[27898784496] [[36m-----[0m] [kernel] [CPU0] Initializing tasking...
[27964338534] [[36m-----[0m] [kernel::sched] [CPU0] Scheduler initialized
[27965898741] [[36m-----[0m] [kernel] [CPU0] Initializing VFS...
[
```
</details>
