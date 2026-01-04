Feature: Cursor rendering performance
  Cursor blitting should be fast enough for smooth interaction.

  @cursor @perf @wip
  Scenario: Cursor draw stays under the budget
    Given the system boots successfully
    And the cursor asset "Normal.cur" is loaded
    When Bloom draws 1000 cursor frames
    Then the average cursor blit time is below 200 microseconds

  @cursor @simd @wip
  Scenario: SIMD path is used on x86_64
    Given the system boots successfully on x86_64
    And SIMD features are available
    When Bloom performs alpha blending
    Then the SSE2 fast path should be utilized
