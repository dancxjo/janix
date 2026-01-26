@desktop @drawing
Feature: Custom Drawing

  As a developer
  I want to render custom vector graphics
  So that I can build rich user interfaces

  Scenario: DrawList demo renders shapes and updates
    Given the machine is running
    And I wait for the system to reach ready state
    When I wait for 5 seconds
    Then I should see a window at 100, 100 with background color "#F5F5F0"
    And I should see a pixel at 130, 130 with color "#22AA66"
    When I wait for 2 seconds
    Then I should see a pixel at 130, 130 with color "#AA2244"
