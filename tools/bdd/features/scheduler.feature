Feature: Scheduler liveness and fairness
  The scheduler is preemptive, makes progress, and never deadlocks the system.
  Runnable tasks eventually run; blocked tasks wake when conditions are met.

  @sched @smoke
  Scenario: The scheduler switches tasks over time
    Given the system has reached "kernel ready"
    When I wait for 200 milliseconds
    Then the serial log should contain "TICK:"
    And the serial log should contain "switch"

  @sched @wip
  Scenario: Runnable tasks eventually run
    Given sprout is online
    And there are 3 runnable user tasks
    When I wait for 1 second
    Then each of those tasks should have executed at least once

  @sched @block @wip
  Scenario: A task may block on a deadline watch
    Given sprout is online
    And the task "clock" is runnable
    When "clock" blocks on a deadline of 100 milliseconds
    Then "clock" should become "blocked"
    And another runnable task should run

  @sched @block @wip
  Scenario: A blocked task wakes when its watch fires
    Given the task "clock" is blocked on a deadline
    When the deadline expires
    Then "clock" should become "runnable" within 50 milliseconds
