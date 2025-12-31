Feature: Boot milestones

  Scenario Outline: Kernel prints Booted on serial for <arch>
    Given I boot the OS in qemu for "<arch>"
    Then I expect to see "Booted." in the serial console

    Examples:
      | arch        |
      | x86_64      |
      | aarch64     |
      | riscv64     |
      # | loongarch64 |


