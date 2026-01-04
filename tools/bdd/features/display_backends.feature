Feature: Swappable display backends
  The primary display is exposed as a generic bytespace-backed device.
  Bloom should work unchanged across display providers.

  @gfx @smoke
  Scenario: Limine framebuffer provider exposes a primary display bytespace
    Given I boot the system with display provider "limine_fb"
    Then the serial log should contain "DISPLAY: selected provider limine_fb"
    And the graph should contain a Thing named "device.display.primary"
    And the Thing "device.display.primary" should link to a Thing of kind "kind.Bytespace"
    And the display metadata should include a nonzero width and height

  @gfx @swap @wip
  Scenario: RAMFB provider exposes the same graph contract
    Given I boot the system with display provider "ramfb"
    # RamFB is exposed via UEFI GOP/Limine, so it appears as limine_fb
    Then the serial log should contain "DISPLAY: selected provider limine_fb"
    And the graph should contain a Thing named "device.display.primary"
    And the Thing "device.display.primary" should link to a Thing of kind "kind.Bytespace"

  @gfx @bloom @wip
  Scenario Outline: Bloom can paint a solid color on any provider
    Given I boot the system with display provider "<provider>"
    And sprout is online
    And the process "bloom" has capability "cap.framebuffer"
    When "bloom" paints the primary display "cornflower"
    Then the framebuffer should change within 100 milliseconds

    Examples:
      | provider  |
      | limine_fb |
      | ramfb     |

  @gfx @caps @wip
  Scenario: Bloom cannot access display bytespace without capability
    Given I boot the system with display provider "limine_fb"
    And sprout is online
    And the process "bloom" has no capability "cap.framebuffer"
    When "bloom" requests framebuffer metadata
    Then the syscall should fail with status "EPERM"
