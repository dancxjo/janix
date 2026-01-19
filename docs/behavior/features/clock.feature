@clock
Feature: Clock application
  As a user
  I want to see a digital clock on the screen
  So that I can tell the time

  Scenario: Clock window appears and updates
    Given the machine is running
    When I wait for the system to boot
    And I wait for 5 seconds
    Then I should see a message in the serial output that says "CLOCK: Entering main loop"
    And the log should match pattern "CLOCK PUBLISH: thing=\d+ now_text='\d{2}:\d{2}:\d{2}'"
    And the clock window should be visible
