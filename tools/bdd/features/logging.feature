Feature: Logging is first-class and graph-visible

@core
Scenario Outline: Userspace log appears in kernel output and in graph
  Given I boot ThingOS on "<arch>"
  And a user task apps/logview is started
  When it calls sys_log(level=Info, msg="hello from userspace")
  Then the serial output contains "hello from userspace"
  And the graph contains a node log.0 with Kind kind.LogEvent
  And log.0 links to the calling task.self
  And log.0 records level=Info and the message bytes

  Examples:
    | arch     |
    | x86_64   |
    | aarch64  |
    | riscv64  |
    | loongarch64 |

@core
Scenario Outline: Log ring buffer does not allocate in IRQ context
  Given I boot ThingOS on "<arch>"
  When I generate 10,000 log events rapidly from userspace
  Then the kernel does not OOM
  And no allocation occurs in interrupt context
  And logs may drop with a "dropped=N" counter node in the graph

  Examples:
    | arch     |
    | x86_64   |
    | aarch64  |
    | riscv64  |
    | loongarch64 |
