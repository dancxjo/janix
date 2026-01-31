@creative
Feature: Creative Workflow

  As a system architect
  I want to visualize the system graph
  So that I can understand the relationship between active components

  Scenario: User inspects the system graph
    Given the machine is running
    And I wait for the system to reach ready state
    When I wait for 5 seconds
    Then I should see the "Photosynthesis" application window
    And I should see graph nodes rendered inside the window
