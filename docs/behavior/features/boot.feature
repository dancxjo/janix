@boot
Feature: Boot brings up core subsystems
  As a kernel developer
  I want to verify the kernel boots and reaches scheduler loop
  So that I can validate the v0.5 typed BootRuntime, paging split, heap, and tasking

  Scenario Outline: Kernel boots and reaches scheduler loop on <arch>
    Given the machine is started
    When I wait for the system to reach ready state
    Then the boot log should contain all required signals
    And the system should show liveness

  Examples:
    | arch      |
    | x86_64    |

  Scenario: Lilac Boot Screen
    Given the machine is started
    When I wait for the system to boot
    Then the screen should be filled with "Lilac"
