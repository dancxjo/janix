Feature: Local APIC initialization and timer
  The Local APIC (LAPIC) replaces the legacy PIC for timer interrupts,
  enabling preemptive scheduling on x86_64.

  @lapic @smoke
  Scenario: LAPIC replaces legacy PIC
    Given the system boots on x86_64
    Then the serial log should contain "LAPIC: enabled"
    And the serial log should contain "TIMER: Legacy PIC disabled"

  @lapic @timer
  Scenario: LAPIC timer drives scheduler ticks
    Given the system boots on x86_64
    When I wait for 200 milliseconds
    Then the serial log should contain "TICK:"
    And the serial log should contain "switch"

  @lapic @platform
  Scenario: LAPIC is published to the graph via Platform
    Given the system boots on x86_64
    Then the serial log should contain "PLATFORM: x86_64 initialized"
    And the serial log should contain "PLATFORM: CPU apic_id="
