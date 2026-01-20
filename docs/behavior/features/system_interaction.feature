@interaction @clock
Feature: System Interaction and Feedback

  As a user
  I want the system to provide visual feedback for background processes
  So that I know the system is alive and functioning correctly

  Scenario: Clock updates over time
    When I start the machine
    Then I should see a clock window displaying a ticking clock
    And I should see the system clock tick for several seconds in the serial console
    And the log should match pattern "unix=\d+ utc=.* mono_ns=\d+"
