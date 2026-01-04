Feature: Multi-architecture behavioral parity
  Core behaviors should be consistent across supported architectures.
  If a capability/device is absent, the graph reflects that absence gracefully.

  @multiarch @wip
  Scenario Outline: Boot reaches kernel ready on each architecture
    Given I boot ThingOS on "<arch>"
    Then the system should reach "kernel ready"
    And the system should not panic

    Examples:
      | arch        |
      | x86_64      |
      | aarch64     |
      | riscv64     |
      | loongarch64 |

  @multiarch @wip
  Scenario: Boot reaches kernel ready on each architecture
    Given I boot the system
    Then the system should reach "kernel ready"
    And the system should not panic

  @multiarch @wip
  Scenario: The syscall ABI returns a structured result on each architecture
    Given I boot the system
    When a user task calls syscall "sys_log"
    Then the syscall should return "(status, val0, val1)" without corruption

  @multiarch @graceful @wip
  Scenario: Missing devices are represented as absence, not failure
    Given I boot ThingOS on "riscv64"
    When I debug query for Place "place.devices"
    Then the query should succeed
    And devices that do not exist should simply be absent
