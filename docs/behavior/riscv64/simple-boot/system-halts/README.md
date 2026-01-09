# ❌ Scenario: System halts

> Last run: 2026-01-08 18:45:27

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | When I turn on the machine | ✅ | 2408ms | - [📜](./01/serial.log) |
| 2 | Then I should see a message in the serial output that says "System booted" | ✅ | 823ms | [📷](./02/after.png) - |
| 3 | Then I should see that the machine has halted | ❌ | 29268ms | - - |

📜 [Full Serial Log](./serial.log)
