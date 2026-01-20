@input @keyboard
Feature: Keyboard Input and Symbol Rendering

  As a user
  I want keyboard input to be processed and rendered correctly
  So that I can type and see my input on the screen

  Scenario: Keyboard input appears on screen
    Given the clock window is ticking
    When I press a key
    Then I should see the corresponding character appear in the lower-right corner of the screen

  Scenario: Modifier keys change rendered symbols
    Given the clock window is ticking
    When I press Alt+A
    Then I should see the appropriate symbol rendered
