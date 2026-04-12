# ✅ Scenario: Supervisor spawns child processes without forking

> Last run: 2026-04-12 15:53:37

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is running | ✅ | 9630ms | - - - |
| 2 | Then I should see a message in the serial output that says "SPROUT:" within 120s | ✅ | 0ms | - - - |
| 3 | And the log should not contain "SYS_FORK" | ✅ | 501ms | - [📜](./03/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[24372707700] [[36m-----[0m] [kernel] [CPU0] thing-os kernel starting...
[24839314929] [[36m-----[0m] [bran::arch::x86_64] [CPU0] [SERIAL] UART COM1 initialized (115200 8N1 FIFO-1 IRQ-on)
[24851530341] [[36m-----[0m] [kernel] [CPU0] Initializing global allocator...
[30568551684] [[32mINFO [0m] [kernel::vfs::devfs] [CPU0] devfs: set_boot_fb width=1920 height=1080 pitch=7680 resource_id=0xfb000000
[30590375937] [[36m-----[0m] [kernel] [CPU0] Seeding entropy pool...
[30596145690] [[36m-----[0m] [kernel] [CPU0] Initializing SIMD...
[30600999396] [[36m-----[0m] [kernel] [CPU0] Initializing tasking...
[30667368600] [[36m-----[0m] [kernel::sched] [CPU0] Scheduler initialized
[30668988966] [[36m-----[0m] [kernel] [CPU0] Initializing VFS...
[31068472875] [[36m-----[0m] [kernel] [CPU0] Spawning init process...
[31087661385] [[36m-----[0m] [kernel] [CPU0] Entering scheduler loop.
[31124940066] [[32mINFO [0m] [sprout] [CPU2] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[31314787020] [[32mINFO [0m] [sprout::supervisor] [CPU2] SPROUT: Launching early serial shell...
[31351442595] [[32mINFO [0m] [sprout::supervisor] [CPU2] SPROUT: Fanning out setup pipelines...
[31420765101] [[32mINFO [0m] [sprout::supervisor] [CPU2] SPROUT: Spawning poll_mux verification test...
[31497219336] [[32mINFO [0m] [sprout::supervisor] [CPU2] SPROUT: --- Supervisor Loop Cycle Start (tasks=0) ---
[31501960710] [[32mINFO [0m] [sprout::supervisor] [CPU2] SPROUT: --- Supervisor Loop Cycle End ---
[31505173326] [[32mINFO [0m] [user.print] [CPU3] --- poll_mux start ---
T:1290 [31533936192] [[32mINFO [0m] [user.print] [CPU3] Pipe created: read=3, write=4
[31549164240] [[32mINFO [0m] [user.print] [CPU3] Channel created: h1=3, h2=4, bridged_fd=5
[31554976332] [[32mINFO [0m] [user.print] [CPU3] VFS file opened: fd=6
[31557921978] [[32mINFO [0m] [user.print] [CPU3] Polling pipe+channel for 100ms (should timeout)...
janix sh

```
</details>
