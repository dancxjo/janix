Feature: Pointer state in graph and cursor rendering
  The PS/2 mouse publishes pointer state to the graph as a Pointer Thing.
  Bloom reads this state and renders a hardware-independent cursor.

  @input @pointer @smoke
  Scenario: PS/2 mouse updates the pointer Thing
    Given the system boots successfully
    And the graph contains a Pointer Thing named "pointer.0"
    Then the Pointer Thing should be linked from "place.input"

  @input @pointer @wip
  Scenario: Pointer coordinates stay within framebuffer bounds
    Given the system boots successfully
    And the graph contains a Pointer Thing named "pointer.0"
    When the PS/2 mouse reports movement
    Then the Pointer Thing "x" should be within display width
    And the Pointer Thing "y" should be within display height

  @gfx @cursor @smoke
  Scenario: Bloom renders cursor at pointer position
    Given the system boots successfully
    And Bloom is running and painting
    And a cursor asset "cursor.bmp" exists
    Then Bloom should log "BLOOM: initialized"

  @gfx @cursor @damage @wip
  Scenario: Cursor rendering uses damage rects
    Given the system boots successfully
    And Bloom is running and painting
    When the Pointer moves
    Then only the cursor region should be redrawn
    And no cursor trails remain on screen
