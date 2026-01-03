Feature: Demo app vertical slice
  A user application can allocate heap, draw, receive input, log through syscalls,
  appear in the graph, and exit cleanly.

  @demo @wip
  Scenario: logview starts as a user task with a heap
    Given sprout is online
    When the process "logview" starts
    Then it should allocate heap memory successfully
    And it should appear in the graph as a "kind.Process"

  @demo @wip
  Scenario: logview draws and receives input
    Given "logview" is running
    When "logview" draws a frame
    Then the display should update
    When I press the key "Q"
    Then "logview" should receive a KeyEvent

  @demo @wip
  Scenario: logview logs through syscall and appears in the graph
    Given "logview" is running
    When "logview" logs "hello from logview"
    Then the serial log should contain "hello from logview"
    And the graph should contain a Thing of kind "kind.LogEntry"

  @demo @wip
  Scenario: logview exits without tearing down the world
    Given "logview" is running
    When "logview" exits cleanly
    Then the system should remain running
    And other services should remain online
