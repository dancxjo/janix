@input @keyboard
Feature: Keyboard input and layout

  As a system developer
  I want to verify that keyboard input is received, mapped, and rendered
  So that I can confirm key events flow through the input pipeline

  Scenario: Keypress emits contract log
    Given the clock window is ticking
    When I press a key
    Then the serial log should contain 'CONTRACT: input key_event'

  Scenario: Modifier mapping produces alternate symbol
    Given the clock window is ticking
    When I press Alt+A
    Then the serial log should contain 'CONTRACT: input key_event'
    And I should see the appropriate symbol rendered
