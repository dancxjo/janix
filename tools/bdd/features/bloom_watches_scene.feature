Feature: Bloom watches the scene graph and redraws on watch delivery
  Bloom composes windows only after graph/thing watch events and sleeps on watch_wait otherwise.

  Scenario: Bloom draws a window after it is added to the windows graph
    Given Bloom is running
    When Sprout creates Window "Hello" in graph.windows
    Then Bloom logs receiving GRAPH_MEMBER_ADDED for that window
    And Bloom draws the window
    And Bloom logs registering a thing watch for that window

  Scenario: Bloom updates window position after a watch event
    Given a window exists in graph.windows
    When Sprout updates its rect
    Then Bloom logs THING_UPDATED for that window
    And Bloom redraws with the new rect

  Scenario: Bloom removes a window after it is removed from the graph
    Given a window exists and is drawn
    When Sprout removes it from graph.windows
    Then Bloom logs GRAPH_MEMBER_REMOVED
    And Bloom redraws without it
