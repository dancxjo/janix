@interaction @clock @anchoring
Feature: Clock Anchoring Invariants

  Scenario: No app logs time before anchor
    When I turn on the machine
    Then the log should match pattern "System clock anchored"
    And the log does not contain "[CLOCK] time=" before "System clock anchored"
    And the log does not contain "[CLOCK] unix=" before "System clock anchored"

  Scenario: Clock syscall returns error before anchor
    When I turn on the machine
    Then the serial output should contain "TIME ANCHORING TEST: Success - sys_time_now returned EAGAIN before anchor"
    And the serial output should contain "System clock anchored"
    And the serial output should contain "TIME ANCHORING TEST: Success - sys_time_now returned valid time after anchor"
