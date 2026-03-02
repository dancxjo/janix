Feature: System Capabilities Tour

  As a new user or developer
  I want to take a guided tour of the system's unique architecture and user interface
  So that I can quickly learn how Thing-OS operates as a living graph

  Scenario: Discovering the system as a queryable graph
    Given the machine is running
    And the anther server is ready
    When I execute the GQL query "MATCH (t:proc.Task) RETURN t.name"
    Then the response status should be 200
    And the GQL result should have at least 5 rows
    When I execute the GQL query "MATCH (s:svc.Root)-[:SUPERVISES]->(child) RETURN child.name"
    Then the response status should be 200
    And the GQL result should have at least 3 rows

  Scenario: Exploring the desktop environment components
    Given the machine is running
    When I wait for 15 seconds
    Then I should see the desktop wallpaper
    And I should see the "Clock" application
    And the "Clock" application should be ticking
    And I should see the "Photosynthesis" application window
    And I should see the network status window in the bottom-left corner
