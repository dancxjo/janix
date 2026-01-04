Feature: Boot wallpaper via Limine module

  Scenario: clouds.bmp is provided as a Limine module and appears in the graph
    Given I boot the system
    Then the boot module list contains a module tagged "clouds.bmp"
    And the graph should contain a Thing named "asset.clouds.bmp"
    And the Asset Thing exposes a readable bytespace of non-zero size

  Scenario: The framebuffer is tiled with clouds.bmp
    Given I boot the system
    And the framebuffer bytespace is available
    When the compositor paints the wallpaper from "clouds.bmp"
    Then the top-left pixel matches the decoded pixel at (0, 0)
    And the pixel at (1280, 0) matches the decoded pixel at (0, 0)
    And the pixel at (0, 720) matches the decoded pixel at (0, 0)
