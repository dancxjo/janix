Feature: System Timer

  @time
  Scenario: Monotonic Time
    When I turn on the machine
    And I wait for the system to boot
    Then the serial output should have monotonic timestamps
