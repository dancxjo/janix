@janix @drivers
Feature: Janix Guardrail 2 – Userland-Driver Direction

  As a kernel developer
  I want to verify that hardware drivers live in userspace
  So that the kernel contains no device logic beyond the SYS_DEVICE_* primitives

  Scenario: Device manager (devd) is launched as a userspace process by the supervisor
    Given the machine is running
    Then I should see a message in the serial output that says "SPROUT: Spawned devd" within 120s

  Scenario: Compositor discovers the display device through the VFS, not a kernel callback
    Given the machine is running
    Then I should see a message in the serial output that says "Bloom: Selected display card" within 120s
