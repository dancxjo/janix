@janix @scheduler
Feature: Janix Guardrail 1 – Scheduler-First Execution Model

  As a kernel developer
  I want to verify that the scheduler is the sole authority granting CPU time
  So that all execution units are properly scheduled tasks and nothing runs outside a task context

  Scenario: Kernel initializes the tasking subsystem and enters the scheduler loop
    Given the machine is booting
    Then I should see a message in the serial output that says "Initializing tasking" within 60s
    And I should see a message in the serial output that says "Scheduler initialized" within 60s
    And I should see a message in the serial output that says "Entering scheduler loop" within 60s

  Scenario: Supervisor process runs as a scheduled user task after the kernel scheduler starts
    Given the machine is booting
    Then I should see a message in the serial output that says "Entering scheduler loop" within 60s
    And I should see a message in the serial output that says "SPROUT:" within 120s
