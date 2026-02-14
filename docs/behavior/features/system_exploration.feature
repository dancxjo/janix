Feature: System Exploration via Graph API

  As a developer or system administrator
  I want to inspect the system state using the Graph Query Language (GQL) API
  So that I can verify the topology of hardware, processes, and services without relying on opaque binary structures

  Scenario: Exploring the process tree and memory map
    Given the anther server is ready
    When I execute the GQL query "MATCH (n:proc.Task) RETURN n"
    Then the response status should be 200
    And the response body should contain "proc.Task"

    When I execute the GQL query "MATCH (c:dev.Cpu) RETURN c"
    Then the response status should be 200
    And the response body should contain "dev.Cpu"

    When I execute the GQL query "MATCH (s:svc.Instance) RETURN s"
    Then the response status should be 200
    And the response body should contain "svc.Instance"
