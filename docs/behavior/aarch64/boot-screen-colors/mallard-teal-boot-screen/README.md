# ❌ Scenario: Mallard Teal Boot Screen

> Last run: 2026-01-08 20:43:52

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is started | ✅ | 244ms | <a href="./01/after.png"><img src="./01/after.png" width="150" /></a> [📜](./01/serial.log) [💾](./01/registers.txt) |
| 2 | When I wait for the system to boot | ❌ | 9919ms | - [📜](./02/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
UEFI firmware (version  built at 23:58:55 on Oct  8 2025)
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;031;100t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI Misc Device" from PciRoot(0x0)/Pci(0x3,0x0)
BdsDxe: starting Boot0002 "UEFI Misc Device" from PciRoot(0x0)/Pci(0x3,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H

```
</details>
