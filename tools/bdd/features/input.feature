Feature: Input pipeline and focus model
  Input enters the system as raw device events, is translated in userspace,
  and becomes graph events routed to the focused surface/window.

  @input @wip
  Scenario: Raw scancodes can be read by inputd with capability
    Given sprout is online
    And the process "inputd" has capability "cap.read_input"
    When I press the key "A"
    Then "inputd" should receive at least 1 raw scancode

  @input @translate @wip
  Scenario: inputd publishes KeyEvent Things to the graph
    Given "inputd" is online
    When I press the key "A"
    Then the graph should contain a Thing of kind "kind.KeyEvent"
    And the event should be linked to Place "place.input"

  @input @focus @wip
  Scenario: Focus determines which surface receives TextEvent
    Given sprout is online
    And there are two surfaces "win.left" and "win.right"
    And the graph indicates focus is "win.right"
    When I type the text "hello"
    Then a TextEvent should be delivered to "win.right"
    And no TextEvent should be delivered to "win.left"
