Feature: Graph as the system
  All meaningful system state is represented as Things and relationships.
  Kernel subsystems must not hide state outside the graph without a corresponding Thing.

  @graph @smoke
  Scenario: The graph is queryable at runtime
    Given the system has reached "kernel ready"
    When I query the graph for "graph.root"
    Then I should receive a valid result

  @graph @tasks @wip
  Scenario: Tasks are represented as Things
    Given the system has reached "userland start"
    When I query the graph for Things of kind "kind.Task"
    Then I should see at least 1 task

  @graph @links @wip
  Scenario: A task is linked to its owning process
    Given sprout is online
    When I query the graph for the Thing named "sprout"
    Then it should have at least 1 relationship "owns" to a Thing of kind "kind.Task"

  @graph @devices @wip
  Scenario: Devices appear in graph.devices
    Given the system has reached "kernel ready"
    When I query the graph for graph "graph.devices"
    Then it should contain zero or more Things
    And each Thing in that graph should have a kind
