Feature: Init Contract

  Scenario: Init task is designated and alive
    Given I boot the OS in qemu for "x86_64"
    Then the serial console log must contain the following lines in order:
      | KERNEL: init task designated |
      | KERNEL: init task alive |
    Then the system must reach steady state

  Scenario: Kernel panics when init exits
    Given I boot the OS in qemu for "x86_64" with init that exits
    Then the serial console log must contain the following lines in order:
      | KERNEL PANIC |
      | init task exited |
