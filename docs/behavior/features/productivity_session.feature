@desktop @productivity
Feature: Productivity Session

  As a creative professional
  I want to multitask with different applications
  So that I can be productive with my time and design work

  Scenario: Designer checks time while browsing fonts
    Given the machine is running
    And I wait for the system to reach ready state
    When I wait for 5 seconds
    Then I should see a clock window displaying a ticking clock
    When I wait for 3 clock ticks
    Then I should see a window at 50, 50 with background color "#F5F5F0"
    And I should see text-like pixels inside the window at 70, 70
