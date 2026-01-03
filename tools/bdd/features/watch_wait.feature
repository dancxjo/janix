Feature: Watches and event-driven waiting
  Tasks can wait efficiently on watches rather than busy-waiting.
  Watches can represent time, graph changes, or device events.

  @watch @wip
  Scenario: Waiting on "any watch" returns when one fires
    Given sprout is online
    And the task "logview" has a watch set containing:
      | kind     | detail             |
      | deadline |   250 milliseconds |
      | graph    | place.input change |
    When "logview" waits on "any"
    Then it should return when the first watch fires

  @watch @graph @wip
  Scenario: A graph watch fires when a Thing is added to a Place
    Given sprout is online
    And "inspector" is watching Place "place.devices"
    When a new device Thing is added to "place.devices"
    Then the watch should fire for "inspector"

  @watch @device @wip
  Scenario: A device watch fires on an input event
    Given sprout is online
    And "inputd" is watching for raw input events
    When I press the key "A"
    Then the watch should fire for "inputd"
