Feature: System Navigation and Discovery

  As a new user
  I want to navigate the system interface and discover its features
  So that I can effectively use the OS for my daily tasks

  Scenario: Exploring the desktop environment
    Given the machine is started
    When I wait for the system to reach ready state
    Then I should see the desktop wallpaper
    And I should see a cursor centered on the screen
    And I should see the "Clock" application
    And the "Clock" application should be ticking
