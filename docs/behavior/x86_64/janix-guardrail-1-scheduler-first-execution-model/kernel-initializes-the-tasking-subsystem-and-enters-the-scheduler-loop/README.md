# ✅ Scenario: Kernel initializes the tasking subsystem and enters the scheduler loop

> Last run: 2026-04-12 15:53:37

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booting | ✅ | 1ms | - - - |
| 2 | Then I should see a message in the serial output that says "Initializing tasking" within 60s | ✅ | 9441ms | - - - |
| 3 | And I should see a message in the serial output that says "Scheduler initialized" within 60s | ✅ | 101ms | - - - |
| 4 | And I should see a message in the serial output that says "Entering scheduler loop" within 60s | ✅ | 100ms | - [📜](./04/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[23800881456] [[36m-----[0m] [kernel] [CPU0] thing-os kernel starting...
[24315467088] [[36m-----[0m] [bran::arch::x86_64] [CPU0] [SERIAL] UART COM1 initialized (115200 8N1 FIFO-1 IRQ-on)
[24329631150] [[36m-----[0m] [kernel] [CPU0] Initializing global allocator...
[30442792116] [[32mINFO [0m] [kernel::vfs::devfs] [CPU0] devfs: set_boot_fb width=1920 height=1080 pitch=7680 resource_id=0xfb000000
[30469647615] [[36m-----[0m] [kernel] [CPU0] Seeding entropy pool...
[30476597811] [[36m-----[0m] [kernel] [CPU0] Initializing SIMD...
[30481192731] [[36m-----[0m] [kernel] [CPU0] Initializing tasking...
[30555869586] [[36m-----[0m] [kernel::sched] [CPU0] Scheduler initialized
[30561402564] [[36m-----[0m] [kernel] [CPU0] Initializing VFS...
[
```
</details>
