Feature: System Exploration

  As a curious user
  I want to explore the system graph
  So that I can understand how the OS is structured and verify system state

  Scenario: Discovering the CPU topology
    Given the anther server is ready
    When I execute the GQL query "MATCH (c:dev.Cpu) RETURN c"
    Then the result should contain at least 1 node of kind "dev.Cpu"

  Scenario: Verifying Service Registry
    Given the anther server is ready
    When I execute the GQL query "MATCH (s:svc.Service) RETURN s"
    Then the response status should be 200
    And the response body should contain "svc.Service"
