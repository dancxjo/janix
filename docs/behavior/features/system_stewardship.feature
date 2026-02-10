@steward
Feature: System Stewardship

  As a System Steward
  I want to verify the integrity and responsiveness of the ThingOS system
  So that I can ensure compliance with the ThingOS Constitution (clarity, correctness, responsiveness)

  Scenario: Verify System Integrity and Visual Responsiveness
    Given the machine is booting
    When I wait for the system to reach ready state
    Then the serial output should contain "SPROUT:"
    And the serial output should contain "Supervisor starting"
    And the serial output should contain "bloom: First frame rendered"
    And I should see the desktop wallpaper
    And I should see the "Clock" application
    And the "Clock" application should be ticking
    And I should see the "Font Explorer" application
    And the log should not contain "PANIC"
    And the log should not contain "ERROR"
