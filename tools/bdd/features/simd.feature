Feature: SIMD support

  @simd @boot
  Scenario: Kernel detects and enables SIMD
    Given I boot the system
    Then the serial output contains "SIMD: Detecting x86 features..."
    And the serial output contains "SIMD: SIMD Enabled (Eager)"

  @simd @userspace
  Scenario: Userspace SIMD Check
    Given I boot the system
    Then the serial output contains "SIMD Check Starting"
    And the serial output contains "Syscall Success"
    And the serial output contains "Policy: 1"
