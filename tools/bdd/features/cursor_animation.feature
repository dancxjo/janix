Feature: Animated cursor playback
  .ani files are RIFF containers with multiple cursor frames and timing.

  @cursor @animation @smoke
  Scenario: Animated cursor loads from .ani file
    Given the system boots successfully
    When a .ani cursor asset is available
    Then Bloom should log "BLOOM: initialized"
    And it does not panic

  @cursor @animation @wip
  Scenario: .ani advances frames at the intended rate
    Given the system boots successfully
    And the cursor asset "Working.ani" is loaded
    When 500 milliseconds pass on the monotonic clock
    Then the cursor frame index has advanced at least 1 step

  @cursor @animation @loop @wip
  Scenario: .ani loops without stopping
    Given the system boots successfully
    And the cursor asset "Working.ani" is loaded
    When 10 seconds pass on the monotonic clock
    Then the cursor is still animating
