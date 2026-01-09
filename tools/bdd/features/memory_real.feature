@memory @frame_alloc
Feature: Frame Allocator Verification

  Scenario: Real Frame Allocator initializes and performs sanity checks
    Given the machine is started
    Then the serial output should contain "Initializing Real Frame Allocator..."
    And the serial output should contain "frame_alloc: base="
    And the serial output should contain "frame_alloc: total="
    And the serial output should contain "Running frame_alloc sanity check..."
    And the serial output should contain "frame_alloc: sanity: single ok"
    And the serial output should contain "System halted"
