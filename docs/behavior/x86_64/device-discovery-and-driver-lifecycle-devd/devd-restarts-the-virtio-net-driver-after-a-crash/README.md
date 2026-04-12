# ✅ Scenario: devd restarts the VirtIO-net driver after a crash

> Last run: 2026-04-12 15:53:37

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the system has booted | ⏭️ | 0ms | - - - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[25997987070] [[36m-----[0m] [kernel] [CPU0] thing-os kernel starting...
[26353713078] [[36m-----[0m] [bran::arch::x86_64] [CPU0] [SERIAL] UART COM1 initialized (115200 8N1 FIFO-1 IRQ-on)
[26365774677] [[36m-----[0m] [kernel] [CPU0] Initializing global allocator...
[32098722183] [[32mINFO [0m] [kernel::vfs::devfs] [CPU0] devfs: set_boot_fb width=1920 height=1080 pitch=7680 resource_id=0xfb000000
[32124795120] [[36m-----[0m] [kernel] [CPU0] Seeding entropy pool...
[32139692244] [[36m-----[0m] [kernel] [CPU0] Initializing SIMD...
[32148304782] [[36m-----[0m] [kernel] [CPU0] Initializing tasking...
[32214086982] [[36m-----[0m] [kernel::sched] [CPU0] Scheduler initialized
[32217571881] [[36m-----[0m] [kernel] [CPU0] Initializing VFS...
[32666198070] [[36m-----[0m] [kernel] [CPU0] Spawning init process...
[32704841994] [[36m-----[0m] [kernel] [CPU0] Entering scheduler loop.
[32815850133] [[32mINFO [0m] [sprout] [CPU2] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[33072736719] [[32mINFO [0m] [sprout::supervisor] [CPU2] SPROUT: Launching early serial shell...
[33121404954] [[32mINFO [0m] [sprout::supervisor] [CPU2] SPROUT: Fanning out setup pipelines...
[33219537648] [[32mINFO [0m] [sprout::supervisor] [CPU2] SPROUT: Spawning poll_mux verification test...
[33293502660] [[32mINFO [0m] [sprout::supervisor] [CPU2] SPROUT: --- Supervisor Loop Cycle Start (tasks=0) ---
[33297130119] [[32mINFO [0m] [sprout::supervisor] [CPU2] SPROUT: --- Supervisor Loop Cycle End ---
[33319494483] [[32mINFO [0m] [user.print] [CPU3] --- poll_mux start ---
T:1290 [33339479382] [[32mINFO [0m] [user.print] [CPU3] Pipe created: read=3, write=4
[33357886551] [[32mINFO [0m] [user.print] [CPU3] Channel created: h1=3, h2=4, bridged_fd=5
[33373114632] [[32mINFO [0m] [user.print] [CPU3] VFS file opened: fd=6
[33375015828] [[32mINFO [0m] [user.print] [CPU3] Polling pipe+channel for 100ms (should timeout)...
janix sh

```
</details>
