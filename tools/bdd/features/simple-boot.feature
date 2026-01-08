@smoke
Feature: Simple Boot
  The system should boot successfully and announce itself on serial output.

  Scenario: System boots successfully
    Given I boot the system
    Then I should see a message in the serial output that says "System booted."
