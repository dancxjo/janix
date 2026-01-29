@observability @assets
Feature: System Observability and Asset Management

  As a user or developer
  I want to confirm that the system dynamically loads assets from the graph
  So that I know the graph-based resource management is functioning correctly

  Scenario: Dynamic Asset Loading from Graph
    When I start the machine
    Then I should see a message in the serial output that says "[asset_bank] promoting wallpaper" within 30s
    And I should see a message in the serial output that says "[asset_bank] promoting cursor" within 30s
    And I should see a message in the serial output that says "[asset_bank] promoting font" within 30s
    And I should see the wallpaper on the screen within 250 seconds
    And I should see a cursor centered on the screen
    And I should see the text "thing-os" in the top-left corner of the screen
