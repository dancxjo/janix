Feature: Boot contract and graph presence

@core
Scenario Outline: Kernel boots and publishes the root places
  Given I boot ThingOS on "<arch>"
  When the kernel reaches "init complete"
  Then the graph contains a node place.root
  And the graph contains a node place.devices
  And the graph contains a node place.tasks
  And the graph contains a node place.input
  And the graph contains a node scheduler.main
  And each node has a stable UUID and a Kind

  Examples:
    | arch     |
    | x86_64   |
    | aarch64  |
    | riscv64  |
    | loongarch64 |

@core
Scenario Outline: Boot is self-describing
  Given I boot ThingOS on "<arch>"
  When the kernel publishes boot metadata
  Then the graph contains a node boot.handoff with Kind kind.BootHandoff
  And it links to machine.<arch>
  And it records framebuffer presence (if provided)
  And it records module list (if provided)

  Examples:
    | arch     |
    | x86_64   |
    | aarch64  |
    | riscv64  |
    | loongarch64 |
