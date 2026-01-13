@bloom
Feature: Bloom compositor
  As a UI developer
  I want Bloom to render a simple scene with a cursor
  So I can verify display + input wiring

  Scenario: Bloom shows a cursor
    Given the machine is started
    When I wait for the system to boot
    Then I should see a message in the serial output that says "bloom: frame loop started"
    And the bloom center rectangle should be visible
    And the bloom cursor should be visible
