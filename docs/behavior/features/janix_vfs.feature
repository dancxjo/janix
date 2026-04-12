@janix @vfs
Feature: Janix Guardrail 3 – VFS-First System Surface

  As a kernel developer
  I want to verify that all system resources are exposed through mounted filesystem paths
  So that no service depends on in-kernel graph nodes, ThingId lookups, or UI_CROWN discovery

  Scenario: Kernel mounts the virtual filesystem during boot
    Given the machine is booting
    Then I should see a message in the serial output that says "Initializing VFS" within 60s

  Scenario: Compositor reaches first paint using only VFS and file-descriptor state
    Given the machine is running
    Then I should see a message in the serial output that says "Bloom: Selected display card" within 120s
    And I should see a message in the serial output that says "Bloom: First paint committed" within 120s
