Feature: Surfaces and compositing
  Graphics are expressed through surfaces and a compositor.
  The display is a bytespace-like target so the backend can be swapped.

  @gfx @smoke @wip
  Scenario: Bloom can paint the screen a solid color
    Given sprout is online
    And the process "bloom" has capability "cap.framebuffer"
    When "bloom" paints the primary display "cornflower"
    Then the framebuffer should change within 100 milliseconds

  @gfx @surface @wip
  Scenario: A surface is a Thing with ownership and bounds
    Given sprout is online
    And the process "bloom" has capability "cap.surface"
    When "bloom" creates a surface of size 800 by 600
    Then the graph should contain a Thing of kind "kind.Surface"
    And the surface should have relationships:
      | rel     | target_kind  |
      | ownedBy | kind.Process |
      | inPlace | kind.Place   |

  @gfx @swap @wip
  Scenario: The display backend is swappable
    Given sprout is online
    Given I boot the system with display provider "limine_fb"
    Then bloom should be able to draw
    When I boot the system with display provider "mock_gpu"
    Then bloom should be able to draw
