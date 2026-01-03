Feature: Syscall ABI is canonical and consistent

@core
Scenario Outline: Syscall numbers match the canonical table
  Given I boot ThingOS on "<arch>"
  When a userspace task calls syscall(nr) for each defined syscall number
  Then each syscall returns either Success or a specific "not implemented" error
  And no syscall returns "wrong handler" behavior

  Examples:
    | arch     |
    | x86_64   |
    | aarch64  |
    | riscv64  |
    | loongarch64 |

@core
Scenario Outline: Syscall calling convention returns (status, val0, val1)
  Given I boot ThingOS on "<arch>"
  When a userspace task calls sys_log("probe")
  Then the syscall returns (status=0, val0=*, val1=*)
  And status is the only field interpreted as success/failure
  And val0 and val1 are stable across architectures for that syscall

  Examples:
    | arch     |
    | x86_64   |
    | aarch64  |
    | riscv64  |
    | loongarch64 |
