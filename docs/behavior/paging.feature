Feature: Paging Subsystem
  As a kernel developer
  I want to verify that the paging subsystem works correctly
  So that I can safely map memory

  Scenario: Basic Paging Mechanism
    Given the machine is started
    Then the serial output should contain "Paging subsystem test passed"
