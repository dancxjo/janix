Feature: System Tour

  As a new user
  I want to verify that all my peripherals and core apps are working
  So that I can start my work

  Scenario: The morning check
    Given the machine is running
    When I wait for 5 seconds
    Then I should see the desktop wallpaper
    And I should see a cursor centered on the screen
    And I should see the "Clock" application
    And the "Clock" application should be ticking
    And I should see the network status window in the bottom-left corner
    And I should see the "Photosynthesis" application window
    And I should see graph nodes rendered inside the window

  Scenario: Verifying system services via logs
    Given the machine is running
    Then the serial output should contain "SPROUT: Supervisor starting"
    And the serial output should contain "[bloom] First frame rendered"
    And the serial output should contain "FETCHD: Got IP address"
