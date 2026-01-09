# ✅ Scenario: System halts

> Last run: 2026-01-08 18:56:42

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | When I turn on the machine | ✅ | 1516ms | - - - |
| 2 | Then I should see a message in the serial output that says "System booted" | ✅ | 216ms | [📷](./02/after.png) - [💾](./02/registers.txt) |
| 3 | Then I should see that the machine has halted | ✅ | 3255ms | - [📜](./03/serial.log) - |

📜 [Full Serial Log](./serial.log)
