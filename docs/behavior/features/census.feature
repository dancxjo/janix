Feature: Unified Device Graph Census

  @smoke
  Scenario: Kernel and Sprout Census listing
    Given the machine is running
    Then the log should contain "ROOT DUMP NODES"
    And the log should contain "dev.Host"
    And the log should contain "source: 0"
    And the log should contain "confidence: 1"
    And the log should contain "ROOT DUMP EDGES"
    And the log should contain "[:HAS_BUS]"
    And the log should contain "dev.rtc.Cmos"
