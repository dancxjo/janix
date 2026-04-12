# ✅ Scenario: Kernel creates the init process via spawn, not fork

> Last run: 2026-04-12 15:53:37

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booting | ✅ | 0ms | - - - |
| 2 | Then I should see a message in the serial output that says "Spawning init process" within 60s | ✅ | 9441ms | - - - |
| 3 | And the log should not contain "SYS_FORK" | ✅ | 499ms | - [📜](./03/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[23622740493] [[36m-----[0m] [kernel] [CPU0] thing-os kernel starting...
[24069647019] [[36m-----[0m] [bran::arch::x86_64] [CPU0] [SERIAL] UART COM1 initialized (115200 8N1 FIFO-1 IRQ-on)
[24081810819] [[36m-----[0m] [kernel] [CPU0] Initializing global allocator...
[29933599509] [[32mINFO [0m] [kernel::vfs::devfs] [CPU0] devfs: set_boot_fb width=1920 height=1080 pitch=7680 resource_id=0xfb000000
[29949539169] [[36m-----[0m] [kernel] [CPU0] Seeding entropy pool...
[29953896126] [[36m-----[0m] [kernel] [CPU0] Initializing SIMD...
[29956424256] [[36m-----[0m] [kernel] [CPU0] Initializing tasking...
[30001269870] [[36m-----[0m] [kernel::sched] [CPU0] Scheduler initialized
[30002651811] [[36m-----[0m] [kernel] [CPU0] Initializing VFS...
[30347680638] [[36m-----[0m] [kernel] [CPU0] Spawning init process...
[30361106655] [[36m-----[0m] [kernel] [CPU0] Entering scheduler loop.
[30412842273] [[32mINFO [0m] [sprout] [CPU2] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[30600818622] [[32mINFO [0m] [sprout::supervisor] [CPU2] SPROUT: Launching early serial shell...
[30629375007] [[32mINFO [0m] [sprout::supervisor] [CPU2] SPROUT: Fanning out setup pipelines...
[30688747089] [[32mINFO [0m] [sprout::supervisor] [CPU2] SPROUT: Spawning poll_mux verification test...
[30753484575] [[32mINFO [0m] [sprout::supervisor] [CPU2] SPROUT: --- Supervisor Loop Cycle Start (tasks=0) ---
[30760775562] [[32mINFO [0m] [sprout::supervisor] [CPU2] SPROUT: --- Supervisor Loop Cycle End ---
[30768706518] [[32mINFO [0m] [user.print] [CPU3] --- poll_mux start ---
T:1290 [30786056862] [[32mINFO [0m] [user.print] [CPU3] Pipe created: read=3, write=4
[30797153178] [[32mINFO [0m] [user.print] [CPU3] Channel created: h1=3, h2=4, bridged_fd=5
[30809480427] [[32mINFO [0m] [user.print] [CPU3] VFS file opened: fd=6
[30811932096] [[32mINFO [0m] [user.print] [CPU3] Polling pipe+channel for 100ms (should timeout)...
janix sh

```
</details>
