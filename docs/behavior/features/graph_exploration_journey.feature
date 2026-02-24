Feature: Graph Exploration Journey

  As a system administrator or curious user
  I want to explore the system graph using GQL
  So that I can understand the running state of the OS

  Scenario: Discovering Hardware Resources
    Given the machine is running
    And the anther server is ready
    When I execute the GQL query "MATCH (n:dev.Cpu) RETURN n"
    Then the GQL result should have at least 1 row
    And row 0 column 0 of the GQL result should be a node

  Scenario: Inspecting Running Processes
    Given the machine is running
    And the anther server is ready
    When I execute the GQL query "MATCH (n:proc.Task) RETURN n"
    Then the GQL result should have at least 1 row
