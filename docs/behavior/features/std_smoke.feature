@std @smoke
Feature: Rust std in userland

  As a system developer
  I want Rust std programs to run on ThingOS
  So that normal Rust applications can execute without no_std

  Scenario: std smoke test runs and logs output
    When I start the machine
    Then I should see a message in the serial output that says "std_smoke: hello from std" within 20s
    And I should see a message in the serial output that says "std_smoke: read back 'thingos std'" within 20s
    And I should see a message in the serial output that says "std_smoke: thread 4" within 20s
    And I should see a message in the serial output that says "std_smoke: done" within 20s
