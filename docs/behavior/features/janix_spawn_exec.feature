@janix @spawn
Feature: Janix Guardrail 4 – Spawn and Exec Process Model

  As a kernel developer
  I want to verify that new processes are created via spawn+exec and never via fork
  So that there is no copy-on-write address-space duplication in the system

  Scenario: Kernel creates the init process via spawn, not fork
    Given the machine is booting
    Then I should see a message in the serial output that says "Spawning init process" within 60s
    And the log should not contain "SYS_FORK"

  Scenario: Supervisor spawns child processes without forking
    Given the machine is running
    Then I should see a message in the serial output that says "SPROUT:" within 120s
    And the log should not contain "SYS_FORK"
