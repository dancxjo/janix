# ❌ Scenario: System halts

> Last run: 2026-01-08 18:55:39

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | When I turn on the machine | ✅ | 274ms | [📷](./01/after.png) [📜](./01/serial.log) [💾](./01/registers.txt) |
| 2 | Then I should see a message in the serial output that says "System booted" | ✅ | 190ms | [📷](./02/after.png) - [💾](./02/registers.txt) |
| 3 | Then I should see that the machine has halted | ❌ | 29654ms | - [📜](./03/serial.log) - |

📜 [Full Serial Log](./serial.log)
