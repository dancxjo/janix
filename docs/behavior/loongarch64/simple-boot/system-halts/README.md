# ✅ Scenario: System halts

> Last run: 2026-01-08 18:49:27

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | When I turn on the machine | ✅ | 797ms | [📷](./01/after.png) - |
| 2 | Then I should see a message in the serial output that says "System booted" | ✅ | 807ms | [📷](./02/after.png) - |
| 3 | Then I should see that the machine has halted | ✅ | 3350ms | - [📜](./03/serial.log) |

📜 [Full Serial Log](./serial.log)
