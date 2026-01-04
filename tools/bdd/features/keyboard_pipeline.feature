Feature: Keyboard input pipeline
  The keyboard input pipeline handles the journey of a keypress from hardware interrupt
  to graph-native event delivery.

  Background:
    Given the system has booted
    And the scheduler is running with interrupts enabled
    And the input service is available

  @presence
  Scenario: The system exposes a keyboard-capable input device when available
    Given at least one keyboard-capable input device exists
    When I query the input devices
    Then I can discover a device with kind "Keyboard"
    And it has a stable identity

  @presence
  Scenario: The system still boots without a keyboard
    Given no keyboard-capable input device exists
    When the system is running
    Then the input service remains healthy
    And no keyboard events are produced

  @events
  Scenario: Key activity produces raw key events
    When a key is pressed
    Then a KeyEvent is produced
    And the event includes press-or-release state
    And the event includes the source device identity

  @events
  Scenario: Keyboard events are buffered during bursts
    When many keys are pressed rapidly
    Then key events are queued without loss
    And the system remains responsive

  @events
  Scenario: No heavy work is required to capture key events
    When a key event is captured
    Then the capture path does not allocate memory
    And the capture path does not perform layout translation

  @delivery
  Scenario: Key events are deliverable to user programs
    Given an application subscribes to keyboard events
    When a key is pressed
    Then the application receives the KeyEvent

  @delivery
  Scenario: Delivery is non-blocking
    Given an application is not currently reading input
    When key events occur
    Then the events remain available until read

  @delivery
  Scenario: Multiple consumers can observe raw key events
    Given two applications subscribe to keyboard events
    When a key is pressed
    Then both applications can observe the KeyEvent
    And the event identity is consistent between observers

  @focus @future
  Scenario: Focus determines which window receives keyboard events
    Given two windows exist
    And one window is focused
    When a key is pressed
    Then the focused window receives the KeyEvent
    And the unfocused window does not

  @focus @future
  Scenario: Changing focus changes routing immediately
    Given a focused window exists
    When focus changes to another window
    Then subsequent keyboard events route to the new focused window

  @layout @future
  Scenario: A layout service can convert key events to text events
    Given a keyboard layout is active
    When key events correspond to a printable character
    Then a TextEvent is produced

  @layout @future
  Scenario: Non-printing keys do not produce text
    Given a keyboard layout is active
    When the key event is a control or navigation key
    Then no TextEvent is produced

  @observability
  Scenario: Keyboard events are observable as Things
    When a key is pressed
    Then a KeyEvent Thing exists in the graph
    And it links to the originating keyboard device Thing

  @observability
  Scenario: Delivery can be traced
    Given a KeyEvent exists
    When I inspect its relationships
    Then I can trace which consumer received it
