Feature: Boot milestones
  The system MUST narrate its own birth clearly, deterministically, and honestly.

  Scenario Outline: Strict boot ordering for <arch>
    Given I boot the OS in qemu for "<arch>"
    Then the serial console log must contain the following lines in order:
      | BRAN: starting                |
      | BRAN: handoff to kernel       |
      | KERNEL: handoff accepted      |
      | LOG: serial backend installed |
      | KERNEL: symbols init          |
      | KERNEL: graph init            |
      | KERNEL: scheduler init        |
      | KERNEL: spawning sprout       |
      | Booted.                       |

    Examples:
      | arch    |
      | x86_64  |
      | aarch64 |

  Scenario Outline: Kernel refuses premature execution on <arch>
    Given I boot the OS in qemu for "<arch>"
    Then I expect NOT to see "KERNEL: symbols init" before "LOG: serial backend installed"
    And I expect NOT to see "KERNEL: graph init" before "KERNEL: symbols init"

    Examples:
      | arch        |
      | x86_64      |
      | aarch64     |
      | riscv64     |
      | loongarch64 |

  Scenario Outline: Degraded early logging on <arch>
    Given I boot the OS in qemu for "<arch>"
    Then the serial console MAY miss any line starting with "BRAN:"
    But I MUST see "LOG: serial backend installed"
    And after that point, I MUST see all of:
      | KERNEL: symbols init      |
      | KERNEL: graph init        |
      | KERNEL: scheduler init    |
      | KERNEL: spawning sprout   |
      | Booted.                   |

    Examples:
      | arch        |
      | riscv64     |
      | loongarch64 |

  Scenario: Serial backend is installed exactly once
    Given I boot the OS in qemu for "x86_64"
    Then "LOG: serial backend installed" appears exactly once in the serial console
