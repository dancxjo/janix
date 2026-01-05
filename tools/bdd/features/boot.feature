Feature: Boot contract and system bring-up
  The system boots through Bran into the Kernel, then Sprout brings userland online.
  The graph becomes observable early and remains the single source of truth.

  @boot @smoke
  Scenario: Bran hands off a boot contract to the Kernel
    Given I boot the system
    Then the serial output contains "BOOT:"
    And the serial output contains "TICK: switch"
    And the serial output contains "SPROUT: I am alive"
    And it does not panic

  @boot @graph @wip
  Scenario: The Kernel exposes a root graph and a devices graph
    Given the system has reached "kernel ready"
    When I query the graph for the root graph
    Then a graph should exist named "graph.root"
    And a graph should exist named "graph.devices"

  @boot @sprout @wip
  Scenario: Sprout starts and publishes its presence in the graph
    Given the system has reached "userland start"
    Then the graph should contain a Thing of kind "kind.Process"
    And that process should have a name "sprout"
    And the serial log should contain "sprout: online"

  @boot @services @wip
  Scenario: Sprout starts core services
    Given sprout is online
    Then the graph should contain a Thing named "inputd"
    And the graph should contain a Thing named "logview"
