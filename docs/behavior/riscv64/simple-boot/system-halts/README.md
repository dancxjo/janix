# ❌ Scenario: System halts

> Last run: 2026-01-08 18:48:19

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | When I turn on the machine | ✅ | 2413ms | - [📜](./01/serial.log) |
| 2 | Then I should see a message in the serial output that says "System booted" | ✅ | 820ms | [📷](./02/after.png) - |
| 3 | Then I should see that the machine has halted | ❌ | 29273ms | - - |

📜 [Full Serial Log](./serial.log)
