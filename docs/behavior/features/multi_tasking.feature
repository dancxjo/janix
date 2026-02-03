@desktop @multitasking
Feature: Multi-Tasking Environment

  As a power user
  I want to have multiple applications running simultaneously
  So that I can perform different tasks without restarting the system

  Scenario: User verifies multiple active applications
    Given the machine is running
    When I wait for the system to reach ready state
    Then I should see the "Font Explorer" application
    And I should see the "Photosynthesis" application window
    And I should see the "Clock" application
    And the "Clock" application should be ticking
