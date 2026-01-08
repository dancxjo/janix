Feature: Interactive Window Management

  Scenario: Dragging a window moves it
    Given a window 1 at 100, 100 with size 200x200
    When I press left mouse at 150, 110
    And I move mouse to 160, 120
    Then window 1 should receive update rect 110, 110, 200, 200

  Scenario: Resizing a window (bottom-right)
    Given a window 1 at 100, 100 with size 200x200
    When I press left mouse at 298, 298
    And I move mouse to 310, 310
    Then window 1 should receive update rect 100, 100, 212, 212

  Scenario: Resizing respects min limit
    Given a window 1 at 100, 100 with size 100x100
    When I press left mouse at 198, 198
    And I move mouse to 120, 120
    Then window 1 should receive update rect 100, 100, 50, 50
