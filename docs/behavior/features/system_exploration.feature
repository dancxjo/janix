Feature: System Exploration

  As a system architect
  I want to query the system graph via the API
  So that I can verify the runtime state of services

  Scenario: Discovering the system root
    Given the anther server is ready
    When I execute the GQL query "MATCH (n:svc.Root) RETURN n"
    Then the response status should be 200
    And the response body should contain "\"success\":true"
    And the response body should contain "\"type\":\"node\""
    And the response body should contain "\"id\""

  Scenario: Inspecting the Scheduler
    Given the anther server is ready
    When I execute the GQL query "MATCH (n:svc.Scheduler) RETURN n"
    Then the response status should be 200
    And the response body should contain "\"success\":true"
    And the response body should contain "\"type\":\"node\""

  Scenario: Listing running processes
    Given the anther server is ready
    When I execute the GQL query "MATCH (n:proc.Task) RETURN n"
    Then the response status should be 200
    And the response body should contain "\"success\":true"
    # Tasks should be present, so we expect at least one node
    And the response body should contain "\"type\":\"node\""
