Feature: Getting Started with Thing-OS

  As a new user
  I want to explore the system's core capabilities
  So that I can verify the installation and understand the unique architecture

  Scenario: A tour of the desktop environment
    Given the machine is started
    When I wait for the system to reach ready state
    Then I should see log messages on the terminal
    And the serial output should contain "SPROUT: Supervisor starting"
    And I should see the desktop wallpaper
    And I should see a cursor centered on the screen
    And I should see the "Clock" application
    And the "Clock" application should be ticking
    And I should see the "Font Explorer" application
    And I should see the "Photosynthesis" application window
    And I should see graph nodes rendered inside the window
    And I should see the network status window in the bottom-left corner
