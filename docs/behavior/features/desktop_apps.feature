@desktop @apps
Feature: Desktop Applications

  As a user
  I want to use built-in applications
  So that I can be productive and explore the system

  Scenario: Font Explorer displays typography
    Given the machine is running
    And I wait for the system to reach ready state
    When I wait for 5 seconds
    Then I should see a window at 50, 50 with background color "#F5F5F0"
    And I should see text-like pixels inside the window at 70, 70
