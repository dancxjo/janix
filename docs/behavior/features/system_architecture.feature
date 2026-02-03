@system @integrity
Feature: System Service Architecture

  As a system architect
  I want to verify that the microkernel service composition (Sprout, Bristle, Bloom, Blossom) initializes correctly
  So that the system remains stable and functional

  Scenario: Core services and drivers are launched by the supervisor
    Given the machine is booting
    When I wait for the system to reach ready state
    Then the serial output should contain "SPROUT: Supervisor starting..."
    And the serial output should contain "SPROUT: Launching app '/boot/ingestd'"
    And the serial output should contain "SPROUT: Launching app '/boot/fontd'"
    And the serial output should contain "SPROUT: Launching app '/boot/blossom'"
    And the serial output should contain "SPROUT: Launching app '/boot/cambium'"

  Scenario: Graphics subsystem initializes correctly
    Given the machine is running
    Then the serial output should contain "[bloom] First frame rendered"
    And I should see the desktop wallpaper

  Scenario: Binding service reports statistics
    Given the machine is running
    When I wait for 10 seconds
    Then the serial output should contain "[cambium] stats:"
