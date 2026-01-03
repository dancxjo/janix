Feature: One vertical slice demo app behaves like a citizen

@core
Scenario Outline: apps/logview is a complete citizen
  Given I boot ThingOS on "<arch>" with framebuffer
  And the init system starts apps/logview as a user task
  When logview allocates heap memory
  And logview draws UI text to its surface
  And logview receives a keypress event
  And logview logs "got key A" through sys_log
  Then the log appears in serial
  And the log appears in the graph linked to logview’s task
  And logview exits cleanly
  And the kernel keeps running and schedules remaining tasks
  And graph nodes for logview transition to Exited

  Examples:
    | arch     |
    | x86_64   |
    | aarch64  |
    | riscv64  |
    | loongarch64 |

@core
Scenario Outline: Exiting a task does not tear down shared system services
  Given I boot ThingOS on "<arch>"
  And tasks include inputd, compositor/bloom, and apps/logview
  When apps/logview exits
  Then inputd remains running
  And bloom remains running
  And place.devices, place.input, and place.tasks remain intact

  Examples:
    | arch     |
    | x86_64   |
    | aarch64  |
    | riscv64  |
    | loongarch64 |
