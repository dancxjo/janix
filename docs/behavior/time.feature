Feature: System Timer

  @time
  Scenario: Monotonic Time (LoongArch64)
    When I turn on the machine
    And I wait for the system to boot
    Then I should see a message in the serial output that says "System halted"
    And the serial output should have monotonic timestamps
