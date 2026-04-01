Feature: Service Discovery

  As an administrator
  I want to use the Anther server to discover running services
  So that I can verify their runtime status via GQL queries

  Scenario: Looking up core system supervisor task
    Given the machine is running
    And the anther server is ready
    When I execute the GQL query "MATCH (n:proc.Task {name: 'sprout'}) RETURN n"
    Then the GQL result should have at least 1 rows
    And row 0 column 0 of the GQL result should be a node
