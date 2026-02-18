Feature: System Exploration

  As a curious user
  I want to explore the running system both visually and programmatically
  So that I can understand how the graph state corresponds to what I see on screen

  Scenario: Correlating Visual Elements with Graph State
    Given the machine is running
    And the anther server is ready
    When I wait for 15 seconds

    # Visual Verification
    Then I should see the desktop wallpaper
    And I should see the "Clock" application
    And I should see the "Photosynthesis" application window
    And I should see the network status window in the bottom-left corner

    # Graph Verification via API
    When I make a GET request to "/top"
    Then the response status should be 200
    And the response body should contain "clock"
    And the response body should contain "photosynthesis"
    And the response body should contain "fetchd"
