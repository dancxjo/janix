Feature: Boot milestones
  The system MUST narrate its own birth clearly, deterministically, and honestly.

  Scenario Outline: Bootloader Handoff on <arch>
    Given I boot the OS in qemu for "<arch>"
    Then the serial console log must contain the following lines in order:
      | BRAN: starting                                       |
      | BRAN: handoff to kernel                              |
      | KERNEL: handoff accepted                             |
      | KERNEL: machine installed                            |

    Examples:
      | arch        |
      | x86_64      |
      | aarch64     |
      | riscv64     |
      | loongarch64 |

  Scenario Outline: Kernel Initialization on <arch>
    Given I boot the OS in qemu for "<arch>"
    Then the serial console log must contain the following lines in order:
      | LOG: serial backend installed                        |
      | KERNEL: symbols init                                 |
      | KERNEL: place store init                             |

    Examples:
        | arch        |
        | x86_64      |
        | aarch64     |
        | riscv64     |
        | loongarch64 |

  Scenario Outline: Ontology Seeding on <arch>
    Given I boot the OS in qemu for "<arch>"
    Then the serial console log must contain the following lines in order:
      | PLACE: root created                                  |
      | PLACE: seeded framebuffer ontology                   |
      | KERNEL: place store indexes rebuilt                  |
      | PLACE: root contains                                 |
      # | KERNEL: ontology self-test passed                    |

    Examples:
        | arch        |
        | x86_64      |
        | aarch64     |
        | riscv64     |
        | loongarch64 |

  Scenario Outline: Scheduler and Sprout on <arch>
    Given I boot the OS in qemu for "<arch>"
    Then the serial console log must contain the following lines in order:
      | KERNEL: scheduler init                               |
      | SCHED: task created: thing.task.sprout               |
      | KERNEL: spawning module: sprout                      |
      | SCHED: task state set: running                       |
      | userland: SPROUT: I am alive!                        |

    Examples:
      | arch        |
      | x86_64      |
      | aarch64     |
      | riscv64     |
      | loongarch64 |

  Scenario Outline: Kernel refuses premature execution on <arch>
    Given I boot the OS in qemu for "<arch>"
    Then I expect NOT to see "KERNEL: symbols init" before "LOG: serial backend installed"
    And I expect NOT to see "KERNEL: place store init" before "KERNEL: symbols init"
    # And I expect NOT to see "BLOOM: thing created" before "KERNEL: ontology self-test passed"

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
      | KERNEL: place store init  |
      | KERNEL: scheduler init    |
      | KERNEL: spawning sprout   |
    And after that point, I MAY see lines starting with "BLOOM:"
    And after that point, I MAY see lines starting with "userland:"

    Examples:
      | arch        |
      | riscv64     |
      | loongarch64 |

  Scenario: Serial backend is installed exactly once
    Given I boot the OS in qemu for "x86_64"
    Then "LOG: serial backend installed" appears exactly once in the serial console

  Scenario Outline: Sprout world interaction demo for <arch>
    Given I boot the OS in qemu for "<arch>"
    Then the serial console log must contain the following lines in order:
      | KERNEL: spawning sprout                               |
      | SCHED: task state set: running                        |
      | userland: SPROUT: I am alive!                         |

    Examples:
      | arch   |
      | x86_64 |

  Scenario: Framebuffer Ontology
    Given the machine has a display
    When the system boots
    Then "place.devices" should contain "thing.device.display.primary"
    And "thing.device.display.primary" should provide "thing.surface.framebuffer.primary"
    And "thing.surface.framebuffer.primary" should be backed by "thing.bytespace.framebuffer.primary"
