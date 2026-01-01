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
    And no line starting with "KERNEL:" appears before "KERNEL: handoff accepted"

    Examples:
      | arch    |
      | x86_64  |
      | aarch64 |

  Scenario Outline: Kernel refuses premature execution on <arch>
    Given I boot the OS in qemu for "<arch>"
    Then I expect NOT to see "KERNEL: symbols init" before "LOG: serial backend installed"
    And I expect NOT to see "KERNEL: graph init" before "KERNEL: symbols init"
    And I expect NOT to see "Booted." if "KERNEL: spawning sprout" did not occur

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
    And no later log line contains "serial init"

  Scenario Outline: Kernel panic aborts boot on <arch>
    Given I boot the OS in qemu for "<arch>" with "panic_after_graph_init"
    Then I expect to see "KERNEL: graph init"
    And I expect to see "PANIC:"
    And I expect NOT to see "KERNEL: scheduler init"
    And I expect NOT to see "Booted."

    Examples:
      | arch    |
      | x86_64  |
      | aarch64 |

  Scenario Outline: Sprout is the first scheduled entity on <arch>
    Given I boot the OS in qemu for "<arch>"
    Then I expect to see "KERNEL: spawning sprout"
    And I expect to see "SCHEDULER: first task = sprout"
    And I expect NOT to see any "SCHEDULER:" line before "KERNEL: scheduler init"

    Examples:
      | arch    |
      | x86_64  |
      | aarch64 |

  Scenario: Boot milestone summary is emitted atomically
    Given I boot the OS in qemu for "x86_64"
    Then I expect to see a single line starting with "BOOT SUMMARY:"
    And that line must mention:
      | graph     |
      | scheduler |
      | sprout    |

