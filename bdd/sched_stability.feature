Feature: Scheduler Stability

  Scenario: SCHED-COOP-01: 2 tasks alternate via sys_yield() for N turns
    Given a system with 2 tasks
    When tasks alternate via sys_yield() for 100 turns
    Then both tasks should complete successfully
    And the turn count should be 100

  Scenario: SCHED-PREEMPT-01: 2 busy-loop tasks alternate for N ticks
    Given a system with 2 busy-loop tasks
    When tasks run for 100 ticks
    Then both tasks should have received CPU time
    And the task switching should be balanced

  Scenario: SCHED-STACK-01: assert no stack underflow/overflow counters increment
    Given the scheduler is running
    Then all tasks should have stack pointers within their allocated boundaries
    And stack health counters should remain zero

  Scenario: SCHED-IRQ-EOI-01: assert IRQ count increases monotonically and does not wedge
    Given the system timer is active
    When the system runs for 1 second
    Then the IRQ count should have increased monotonically
    And the timer interrupts should not have wedged
