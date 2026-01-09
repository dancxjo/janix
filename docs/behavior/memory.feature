@memory
Feature: Boot Memory Initialization

  Scenario: Kernel successfully initializes allocators and performs early allocations
    Given the machine is started
    Then the serial output should contain "boot: phys ranges="

    And the serial output should contain "System halted"
