Feature: Daily Inspiration

  As a user seeking wisdom
  I want to receive a daily fortune
  So that I can start my day with a thought-provoking message

  Scenario: Receiving a fortune cookie message
    Given the machine is started
    When I wait for 60 seconds
    Then I should see a window at 400, 200 with background color "#FDF5E6"
    And I should see text-like pixels inside the window at 420, 220
