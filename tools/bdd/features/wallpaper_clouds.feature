Feature: Boot wallpaper via Limine module

  @smoke @wallpaper
  Scenario: Bloom renders the cloud wallpaper
    Given I boot the system
    And I wait for "BLOOM: done"
    Then the display should show the clouds wallpaper
