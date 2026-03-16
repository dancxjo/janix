Feature: User Journey: Graph OS

  As a curious user
  I want to explore the underlying system graph of Thing-OS
  So that I can verify my hardware, running processes, and core services are correctly represented as nodes

  Scenario: Verifying Hardware Resources via GQL
    Given the machine is running
    And the anther server is ready
    When I execute the GQL query "MATCH (c:dev.Cpu) RETURN c"
    Then the response status should be 200
    And the GQL result should have at least 1 rows
    And row 0 column 0 of the GQL result should be a node

  Scenario: Inspecting Running Processes via GQL
    Given the machine is running
    And the anther server is ready
    When I execute the GQL query "MATCH (n:proc.Task) RETURN n.name"
    Then the response status should be 200
    And the response body should contain "sprout"
    And the response body should contain "anther"

  Scenario: Discovering Core Services via GQL
    Given the machine is running
    And the anther server is ready
    When I execute the GQL query "MATCH (s:svc.Root) RETURN s"
    Then the response status should be 200
    And the response body should contain "svc.Root"
