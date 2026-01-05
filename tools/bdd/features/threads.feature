Feature: Kernel threads with shared address space
  Threads share an address space and can block/wake without spinning.

  Background:
    Given the system has reached "kernel ready"

  @threads @smoke
  Scenario: Thread spawn is available as syscall
    When I wait for 200 milliseconds
    Then the serial log should contain "SCHED:"
    And the system should have spawned at least one task

  @threads @wip
  Scenario: Thread spawn creates a second schedulable entity
    Given sprout is online
    When a test app spawns a thread
    Then the serial log should contain "create thread"
    And the scheduler should have at least 2 tasks

  @threads @join @wip
  Scenario: Thread join blocks caller until target exits
    Given sprout is online
    When the main thread spawns a worker and calls join
    Then the main thread should become blocked
    And when the worker exits
    Then the main thread should resume

  @threads @watch @wip
  Scenario: Thread can block on watch without CPU spin
    Given a thread is blocked on a watch
    Then the scheduler should report it as Blocked
    When the watch fires
    Then the thread should wake and become runnable
