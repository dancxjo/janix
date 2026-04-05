@desktop @workflow
Feature: Knowledge Worker Routine

  As a knowledge worker
  I want to use my OS to track my projects and monitor my workspace
  So that I can stay organized and productive throughout the day

  Scenario: Morning Workspace Setup
    Given the machine is running
    And the anther server is ready
    When I wait for the system to reach ready state
    And I wait for 5 seconds
    Then I should see the desktop wallpaper
    And I should see a cursor centered on the screen
    And I should see the "Clock" application
    And the "Clock" application should be ticking
    And I should see the "Photosynthesis" application window
    And I should see graph nodes rendered inside the window

  Scenario: Setting Up a New Project in the Graph
    Given the anther server is ready

    # Create the project node
    When I execute the GQL query "CREATE (p:Project {name: \"Project Alpha\", status: \"Planning\"})"
    Then the response status should be 200
    And the response body should contain "success"

    # Create a task node
    When I execute the GQL query "CREATE (t:Task {title: \"Draft Requirements\", assignedTo: \"Me\"})"
    Then the response status should be 200

    # Link task to project
    When I execute the GQL query "MATCH (p:Project {name: \"Project Alpha\"}), (t:Task {title: \"Draft Requirements\"}) CREATE (p)-[:HAS_TASK]->(t)"
    Then the response status should be 200

    # Verify the link
    When I execute the GQL query "MATCH (p:Project {name: \"Project Alpha\"})-[:HAS_TASK]->(t:Task {title: \"Draft Requirements\"}) RETURN t"
    Then the response status should be 200
    And the GQL result should have at least 1 rows
    And row 0 column 0 of the GQL result should be a node

  Scenario: Checking System Resources for Heavy Tasks
    Given the machine is running
    And the anther server is ready
    When I execute the GQL query "MATCH (c:dev.Cpu) RETURN c"
    Then the response status should be 200
    And the GQL result should have at least 1 rows
    And row 0 column 0 of the GQL result should be a node
