Feature: Bytespace and address space mapping

@core
Scenario Outline: Create bytespace and map into userspace
  Given I boot ThingOS on "<arch>"
  When a userspace task calls sys_bytespace_create(size=1MiB, kind=Anon)
  And maps it with sys_space_map(bytespace, vaddr=X, len=1MiB, perms=RW)
  Then writes to X..X+1MiB succeed
  And unmapping with sys_space_unmap(X, len) makes access fault again
  And the graph shows the mapping edges from space.current to bytespace.id

  Examples:
    | arch     |
    | x86_64   |
    | aarch64  |
    | riscv64  |
    | loongarch64 |

@core
Scenario Outline: Mapping permissions are enforced
  Given I boot ThingOS on "<arch>"
  And a userspace task maps a region as ReadOnly
  When it attempts to write to that region
  Then it faults
  And the graph records the permission violation

  Examples:
    | arch     |
    | x86_64   |
    | aarch64  |
    | riscv64  |
    | loongarch64 |
