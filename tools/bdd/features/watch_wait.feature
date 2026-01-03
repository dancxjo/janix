Feature: Watch-based blocking wait
  The scheduler supports a first-class blocked state where a task voluntarily
  becomes non-runnable until one of its graph watches fires (or a timeout occurs).

  Background:
    Given the system has booted to the root Place
    And the scheduler is running at least 2 user tasks

  Scenario: A task can block on watches and stops consuming CPU
    Given a user task "watcher" has registered a watch "w.alpha"
    When the task "watcher" calls wait on watches ["w.alpha"] with WAIT_ANY and no timeout
    Then the task "watcher" transitions to state "Blocked(WatchWait)"
    And the task "watcher" is not present in the runnable set
    And the system continues scheduling other runnable tasks

  Scenario: A watch firing wakes a blocked task
    Given a user task "watcher" is blocked waiting on watches ["w.alpha"] with WAIT_ANY
    When the watch "w.alpha" fires
    Then the task "watcher" transitions to state "Runnable"
    And the scheduler eventually runs the task "watcher"
    And the wait syscall returns WakeReason "WATCH" with watch "w.alpha"

  Scenario: Waiting on multiple watches wakes on any
    Given a user task "watcher" is blocked waiting on watches ["w.alpha", "w.beta"] with WAIT_ANY
    When the watch "w.beta" fires
    Then the task "watcher" transitions to state "Runnable"
    And the wait syscall returns WakeReason "WATCH" with watch "w.beta"

  Scenario: Timeout wakes a blocked task if no watch fires
    Given a user task "watcher" is blocked waiting on watches ["w.alpha"] with WAIT_ANY and timeout 50 ticks
    When 50 ticks elapse without "w.alpha" firing
    Then the task "watcher" transitions to state "Runnable"
    And the wait syscall returns WakeReason "TIMEOUT"

  Scenario: Exiting cleans up wait registrations
    Given a user task "watcher" is blocked waiting on watches ["w.alpha"] with WAIT_ANY
    When the task "watcher" exits
    Then the watch registry has no waiter entries for task "watcher"
    And firing watch "w.alpha" does not reference task "watcher"
