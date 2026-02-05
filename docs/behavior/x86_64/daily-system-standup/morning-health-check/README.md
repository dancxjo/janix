# ❌ Scenario: Morning Health Check

> Last run: 2026-02-05 21:00:10

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is running | ✅ | 12s | |
| 2 | When I wait for the system to reach ready state | ✅ | 1s | |
| 3 | Then I should see a message in the serial output that says "SPROUT: Entering supervisor loop." within 5s | ❌ | 5s | [📸 Timeout](./03/timeout.png) |
| 4 | And the system dashboard should show a balanced layout | ⏭️ | - | |
