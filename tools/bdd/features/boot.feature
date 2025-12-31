Feature: Boot milestones

  Scenario Outline: Full boot verification for <arch>
    Given I boot the OS in qemu for "<arch>"
    Then I expect to see "BRAN: starting" in the serial console
    And I expect to see "BRAN: handoff to kernel" in the serial console
    And I expect to see "KERNEL: handoff accepted" in the serial console
    And I expect to see "LOG: serial backend installed" in the serial console
    And I expect to see "KERNEL: symbols init" in the serial console
    And I expect to see "KERNEL: graph init" in the serial console
    And I expect to see "KERNEL: scheduler init" in the serial console
    And I expect to see "KERNEL: spawning sprout" in the serial console
    And I expect to see "Booted." in the serial console

    Examples:
      | arch        |
      | x86_64      |
      | aarch64     |

  Scenario Outline: Kernel boot verification for <arch>
    Given I boot the OS in qemu for "<arch>"
    # Skip Bran/Early logs as early_putc is unverified on these archs
    Then I expect to see "LOG: serial backend installed" in the serial console
    And I expect to see "KERNEL: symbols init" in the serial console
    And I expect to see "KERNEL: graph init" in the serial console
    And I expect to see "KERNEL: scheduler init" in the serial console
    And I expect to see "KERNEL: spawning sprout" in the serial console
    And I expect to see "Booted." in the serial console

    Examples:
      | arch        |
      | riscv64     |
      | loongarch64 |


