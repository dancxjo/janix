Feature: Scheduler is preemptive and graph-represented

@core
Scenario Outline: Preemptive timer causes task switching
  Given I boot ThingOS on "<arch>"
  And there are 4 runnable tasks task.1..task.4
  When the timer interrupt fires repeatedly
  Then the running task changes at least 10 times
  And the graph updates scheduler.main --[running]--> task.next accordingly
  And each task accumulates cpu.ticks in the graph

  Examples:
    | arch     |
    | x86_64   |
    | aarch64  |
    | riscv64  |
    | loongarch64 |

@core
Scenario Outline: Sleeping task is not scheduled
  Given I boot ThingOS on "<arch>"
  And task A calls sys_sleep(50ms)
  When 10 timer ticks pass
  Then task A is not selected as running during its sleep
  And the graph shows taskA --[state]--> Sleeping
  And after 50ms it becomes Runnable again

  Examples:
    | arch     |
    | x86_64   |
    | aarch64  |
    | riscv64  |
    | loongarch64 |

@nonfatal @core
Scenario Outline: A crashing task does not crash the kernel
  Given I boot ThingOS on "<arch>"
  And task B intentionally triggers a fault (invalid memory access)
  When the fault occurs
  Then task B transitions to Crashed
  And the kernel remains alive
  And other tasks continue switching
  And the graph contains a fault.current node linked to task B with the fault reason

  Examples:
    | arch     |
    | x86_64   |
    | aarch64  |
    | riscv64  |
    | loongarch64 |
