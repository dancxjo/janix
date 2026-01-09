Feature: Paging Subsystem

  @x86_64
  Scenario: Paging Subsystem Self-Test
    Given the machine is started
    Then the serial output should contain "Testing paging subsystem..."
    And the serial output should contain "Switched to new address space"
    And the serial output should contain "Paging subsystem test passed"
