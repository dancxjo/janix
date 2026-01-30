@boot @experience
Feature: First Run Experience

  As a new user
  I want to explore the default desktop environment
  So that I can understand what applications are available and verify the system is working

  Scenario: User explores the desktop and verifies applications
    Given the machine is running
    When I wait for 5 seconds
    Then I should see the desktop wallpaper
    And I should see the "Font Explorer" application
    And I should see the "Clock" application
    And the "Clock" application should be ticking
