Feature: EventStream bytespace messaging

  Scenario: EventStream exists for mouse and advances sequence
    Given the system boots
    Then the graph contains a Thing of kind "EventStream" named "event_stream.mouse"
    And "event_stream.mouse" links to a bytespace
    When the mouse moves
    Then the EventStream "write_seq" increases
    And the EventStream "dropped" does not increase

  Scenario: Bloom consumes pointer events without graph locks
    Given Bloom is running
    And the mouse EventStream exists
    When the mouse moves
    Then Bloom logs "pointer event"
    And Bloom does not query the graph more than once per second
