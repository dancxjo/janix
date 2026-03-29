Feature: Understanding the Graph

  As a new user or developer
  I want to understand the foundational concept of Thing-OS where everything is a node in a graph
  So that I can effectively interact with the system using Graph Query Language (GQL)

  Scenario: Discovering the system hardware
    Given the machine is running
    And the anther server is ready
    When I execute the GQL query "MATCH (c:dev.Cpu) RETURN c"
    Then the response status should be 200
    And the response body should contain "dev.Cpu"
    And the GQL result should have at least 1 rows
    And row 0 column 0 of the GQL result should be a node

  Scenario: Exploring running tasks
    Given the machine is running
    And the anther server is ready
    When I execute the GQL query "MATCH (n:proc.Task) RETURN n.name"
    Then the response status should be 200
    And the response body should contain "sprout"
    And the response body should contain "anther"
    And the GQL result should have at least 1 rows
    And row 0 column 0 of the GQL result should be string "sprout"
