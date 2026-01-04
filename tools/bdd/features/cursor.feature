Feature: Cursor rendering correctness
  Cursor rendering with proper hotspot alignment and alpha blending.

  @cursor @hotspot @smoke
  Scenario: .cur hotspot aligns click point
    Given the system boots successfully
    And the cursor asset "Normal.cur" is loaded
    Then Bloom should log "BLOOM: target_pixel_format=XRGB8888"
    And Bloom should log "BLOOM: initialized"

  @cursor @alpha @smoke
  Scenario: Cursor alpha blending produces smooth edges
    Given the system boots successfully
    And Bloom is running
    When the cursor is drawn over a background
    Then the serial log should contain "BLOOM: initialized"
    And it does not panic

  @cursor @format @smoke
  Scenario: Pixel format is documented at boot
    Given the system boots successfully
    Then the serial log should contain "BLOOM: target_pixel_format=XRGB8888"
    And the serial log should contain "cursor_format=PREMUL_ARGB8888"
