Feature: GraphViewer smoke test
  A quick, thorough proof that Bloom + input + app state + graph summary output are wired.

  Background:
    Given the system boots to Sprout
    And Bloom is running as a userspace compositor

  Scenario: Initial UI invites user and shows output label + button
    Given an app "graphviewer" is started as a user task
    Then I should see a label containing "Welcome"
    And I should see a label containing "press"
    And I should see a label containing "button"
    And I should see a button labeled "Show current graph"
    And the output label should contain "(graph not loaded"

  Scenario: Clicking the button loads a short OpenGQL summary into the output label
    Given an app "graphviewer" is started as a user task
    And the output label contains "(graph not loaded"
    When I wait for the graph summary to load
    Then the output label should contain "MATCH"
    And the output label should contain "RETURN"
    And the output label should contain "nodes"

  Scenario: graphviewer publishes a Window Thing with a widget tree and Bloom renders it
    Given an app "graphviewer" is started as a user task
    When "graphviewer" creates a Window Thing sized 520x340 titled "GraphViewer"
    And "graphviewer" creates a Layout Thing of kind "Column" with padding 16 and gap 12
    And "graphviewer" creates a Label Thing with text "Welcome. Press the button..."
    And "graphviewer" creates a Button Thing with text "Show current graph"
    And "graphviewer" links the Layout as the Window content root
    And "graphviewer" links the widgets as children of the Layout
    And "graphviewer" commits a new Frame Thing for the Window
    Then Bloom must observe the Window Thing and its widget tree
    And Bloom must render the Window rectangle with rounded corners
    And Bloom must render the Label and Button positioned by the Layout
