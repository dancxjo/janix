Feature: System Exploration

  As a system administrator or developer
  I want to interrogate the system graph using GQL queries
  So that I can verify the state of hardware, processes, and services without relying on opaque tools

  Scenario: A developer discovers the CPU topology
    Given the anther server is ready
    When I execute the GQL query "MATCH (c:dev.Cpu) RETURN c"
    Then the response status should be 200
    And the response body should contain "dev.Cpu"
    And the response body should contain "\"kind\":\"dev.Cpu\""

  Scenario: A system administrator audits running processes
    Given the anther server is ready
    When I execute the GQL query "MATCH (t:proc.Task) RETURN t"
    Then the response status should be 200
    And the response body should contain "proc.Task"
    And the response body should contain "\"state\":\"Running\""

  Scenario: Verifying the root service exists
    Given the anther server is ready
    When I execute the GQL query "MATCH (s:svc.Root) RETURN s"
    Then the response status should be 200
    And the response body should contain "svc.Root"
