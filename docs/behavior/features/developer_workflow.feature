@developer
Feature: Developer Workflow

  As a developer
  I want to inspect the system graph via GQL
  So that I can verify the state of the running system and debug issues

  Scenario: Querying System Processes
    Given the machine is running
    And the anther server is ready
    When I execute the GQL query "MATCH (n:proc.Task) RETURN n.name"
    Then the response status should be 200
    And the response body should contain "sprout"
    And the response body should contain "anther"

  Scenario: Inspecting CPU Configuration
    Given the machine is running
    And the anther server is ready
    When I execute the GQL query "MATCH (c:dev.Cpu) RETURN c"
    Then the response status should be 200
    And the response body should contain "dev.Cpu"

  Scenario: Verifying System Services
    Given the machine is running
    And the anther server is ready
    When I execute the GQL query "MATCH (s:svc.Root) RETURN s"
    Then the response status should be 200
    And the response body should contain "svc.Root"
