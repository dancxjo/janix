@perf @clock
Feature: Performance Invariants

  As a developer
  I want steady-state UI updates to be cheap
  So regressions show up quickly

  Scenario: Clock tick should be cheap
    When I start the machine
    Given the clock window is ticking
    And I wait for 10 clock ticks
    Then watch overflows should be 0
    And dirty nodes layout should stay below 4
    And ui.snap.traverse_all should be absent or below 0.05 ms
    And average frame time should be below 25 ms
