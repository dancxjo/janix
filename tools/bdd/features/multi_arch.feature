Feature: Multi-architecture parity

@core
Scenario Outline: Core behaviors are consistent across architectures
  Given I boot ThingOS on "<arch>"
  When I run the "vertical slice" test suite
  Then all scenarios tagged @core pass

  Examples:
    | arch     |
    | x86_64   |
    | aarch64  |
    | riscv64  |
    | loongarch64 |

@core
Scenario Outline: Architecture-specific differences are explicitly declared
  Given I run tests on "<arch>"
  When a feature is not supported (e.g., PS/2 keyboard on non-x86)
  Then the scenario is skipped with a reason recorded
  And the skip reason is a first-class test artifact

  Examples:
    | arch     |
    | x86_64   |
    | aarch64  |
    | riscv64  |
    | loongarch64 |

@perf @core
Scenario Outline: Sustained scheduling and logging does not exhaust memory
  Given I boot ThingOS on "<arch>"
  And 8 user tasks run busy loops with periodic sys_log
  When the system runs for 10 seconds
  Then it does not OOM
  And it does not panic
  And memory usage growth stays under a fixed threshold
  And the graph contains allocator telemetry

  Examples:
    | arch     |
    | x86_64   |
    | aarch64  |
    | riscv64  |
    | loongarch64 |
