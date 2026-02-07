Feature: User Documentation

  As a new user
  I want to understand the capabilities of the system
  So that I can effectively use the OS

  Scenario: Boot and System Verification
    Given the machine is booting
    When I wait for the system to reach ready state
    Then the serial output should contain "SPROUT:"
    And the serial output should contain "Supervisor starting"

  Scenario: Desktop Environment Verification
    Given the machine is running
    When I wait for 10 seconds
    Then I should see the desktop wallpaper
    And I should see a cursor centered on the screen

  Scenario: Launching Core Applications
    Given the machine is running
    When I wait for 10 seconds
    Then I should see the "Font Explorer" application
    And I should see the "Photosynthesis" application window
