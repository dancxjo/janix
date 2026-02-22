@investigate
Feature: Investigating System Processes

  As a system administrator
  I want to verify that critical system processes are running and correctly represented in the graph
  So that I can trust the system state

  Scenario: Verify the Clock Process
    Given the machine is running
    And the anther server is ready
    When I wait for 5 seconds

    # Visual Check
    Then I should see a clock window displaying a ticking clock

    # Graph Check via API (POST)
    When I execute the GQL query "MATCH (n {name: '/clock'}) RETURN n"
    Then the response status should be 200
    And the response body should contain "clock"
    And the response body should contain "pid"

  Scenario: Verify the Task Manager Process
    Given the machine is running
    And the anther server is ready

    # Check for taskman process
    When I execute the GQL query "MATCH (n {name: '/taskman'}) RETURN n"
    Then the response status should be 200
    And the response body should contain "taskman"
