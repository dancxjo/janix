Feature: Userspace heap and memory isolation

@core
Scenario Outline: Userspace can allocate heap and write within it
  Given I boot ThingOS on "<arch>"
  And task C has a userspace heap enabled
  When task C allocates 64 KiB and writes a pattern
  Then task C reads back the same pattern
  And the kernel remains stable
  And the graph records a bytespace.heap for task C’s heap

  Examples:
    | arch     |
    | x86_64   |
    | aarch64  |
    | riscv64  |
    | loongarch64 |

@core
Scenario Outline: Userspace cannot write kernel memory
  Given I boot ThingOS on "<arch>"
  When a userspace task attempts to write to a kernel address
  Then the task faults
  And the kernel survives
  And the fault is recorded in the graph with Kind kind.PageFault

  Examples:
    | arch     |
    | x86_64   |
    | aarch64  |
    | riscv64  |
    | loongarch64 |

@core
Scenario Outline: Two tasks cannot scribble over each other
  Given I boot ThingOS on "<arch>"
  And task D allocates memory and writes "D"
  And task E allocates memory and writes "E"
  When each task attempts to read the other’s memory by guessing addresses
  Then both attempts fail with a fault or access denied
  And neither task’s own heap contents are corrupted

  Examples:
    | arch     |
    | x86_64   |
    | aarch64  |
    | riscv64  |
    | loongarch64 |
