@system @health
Feature: System Health Verification

  As a system administrator or new user
  I want to verify the health of key system subsystems
  So that I can ensure the OS is functioning correctly and is ready for use

  Scenario: Network Stack Initialization
    Given the machine is running
    When I wait for the system to reach ready state
    Then the serial output should contain "NETD: Starting network stack service..."
    And the serial output should contain "NETD: Network stack ready"
    And the serial output should contain "FETCHD: Got IP address"

  Scenario: Graphics Subsystem Ready
    Given the machine is running
    When I wait for the system to reach ready state
    Then the serial output should contain "[bloom] First frame rendered"
    And I should see the desktop wallpaper

  Scenario: Input Subsystem Ready
    Given the machine is running
    When I wait for the system to reach ready state
    Then I should see a cursor centered on the screen

  Scenario: System Time matches Wall Clock
    Given the machine is running
    When I wait for the system to reach ready state
    Then I should see a clock window displaying a ticking clock
    When I wait for 3 clock ticks
    Then the "Clock" application should be ticking
