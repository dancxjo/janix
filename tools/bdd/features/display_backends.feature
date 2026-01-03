Feature: Swappable display backends
  The primary display is exposed as a generic bytespace-backed device.
  Bloom should work unchanged across display providers.

  @gfx @smoke
  Scenario: Limine framebuffer provider exposes a primary display bytespace
    Given I boot ThingOS on "x86_64" with display provider "limine_fb"
    Then the serial log should contain "DISPLAY: selected provider limine_fb"
    And the graph should contain a Thing named "device.display.primary"
    And the Thing "device.display.primary" should link to a Thing of kind "kind.Bytespace"
    And the display metadata should include a nonzero width and height

  @gfx @swap @wip
  Scenario: RAMFB provider exposes the same graph contract
    Given I boot ThingOS on "aarch64" with display provider "ramfb"
    Then the serial log should contain "DISPLAY: selected provider ramfb"
    And the graph should contain a Thing named "device.display.primary"
    And the Thing "device.display.primary" should link to a Thing of kind "kind.Bytespace"

  @gfx @bloom @wip
  Scenario Outline: Bloom can paint a solid color on any provider
    Given I boot ThingOS on "<arch>" with display provider "<provider>"
    And sprout is online
    And the process "bloom" has capability "cap.framebuffer"
    When "bloom" paints the primary display "cornflower"
    Then the framebuffer should change within 100 milliseconds

    Examples:
      | arch    | provider  |
      | x86_64  | limine_fb |
      | x86_64  | ramfb     |
      | aarch64 | ramfb     |

  @gfx @caps @wip
  Scenario: Bloom cannot access display bytespace without capability
    Given I boot ThingOS on "x86_64" with display provider "limine_fb"
    And sprout is online
    And the process "bloom" has no capability "cap.framebuffer"
    When "bloom" requests framebuffer metadata
    Then the syscall should fail with status "EPERM"
