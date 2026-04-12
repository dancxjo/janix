@boot @smoke
Feature: System Boot

  As a kernel developer
  I want to verify that Thing-OS boots successfully
  So that I can confirm the kernel initializes, runs the scheduler, and produces serial output

  Scenario: Boot produces serial log output with monotonic timestamps
    Given the machine is booting
    Then I should see a message in the serial output that says "thing-os kernel starting" within 60s
    And I should see log messages on the terminal
    And each log message should include a monotonically increasing timestamp
    And I should see the system clock tick for several seconds in the serial console
