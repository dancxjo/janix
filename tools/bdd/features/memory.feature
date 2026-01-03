Feature: Bytespaces and address spaces
  Memory is represented as bytespaces that can be mapped into address spaces with permissions.
  Ownership and lifetime are explicit and do not leak.

  @mem @wip
  Scenario: A process can create a bytespace and map it with permissions
    Given sprout is online
    And the process "logview" has capability "cap.map_memory"
    When "logview" creates a bytespace of size 65536
    And "logview" maps the bytespace read-write into its address space
    Then the mapping should succeed
    And the graph should contain a Thing of kind "kind.Bytespace"

  @mem @wip
  Scenario: Unmapping removes access
    Given "logview" has a mapped bytespace
    When "logview" unmaps that range
    Then reads from that range should fault or fail safely

  @mem @lifetime @wip
  Scenario: Task stacks have explicit ownership and do not leak
    Given sprout is online
    When I spawn and exit 1000 short-lived tasks
    Then the kernel should not run out of memory
    And the graph should not retain orphaned "kind.Stack" Things
