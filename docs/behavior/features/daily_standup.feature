@daily @report
Feature: Daily System Standup

  As a lead engineer
  I want to verify the system health and layout at a glance
  So that I can ensure the daily build is stable

  Scenario: Morning Health Check
    Given the machine is running
    When I wait for the system to reach ready state
    Then I should see a message in the serial output that says "SPROUT: Entering supervisor loop." within 5s
    And the system dashboard should show a balanced layout
